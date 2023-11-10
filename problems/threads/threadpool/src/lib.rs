#![forbid(unsafe_code)]
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::thread::JoinHandle;

pub enum Message<Job> {
    NewJob(Job),
    Terminate,
}

pub struct ThreadTask {
    thread: Option<JoinHandle<()>>,
}

impl ThreadTask {
    pub fn new<Job>(id: usize, receiver: Arc<Mutex<Receiver<Message<Job>>>>) -> ThreadTask
    where
        Job: Send + FnOnce() + 'static,
    {
        let thread = std::thread::Builder::new()
            .name(format!("Task {id}"))
            .spawn(move || loop {
                let rec = receiver.lock().unwrap();
                let msg = rec.recv().unwrap();
                drop(rec);
                println!("Thread {id} received task");
                match msg {
                    Message::NewJob(job) => {
                        job();
                        println!("Thread {id} completed task");
                    }
                    Message::Terminate => {
                        println!("Thread {id} terminated");
                        break;
                    }
                }
            })
            .unwrap();

        ThreadTask {
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool<Job> {
    tasks: Vec<ThreadTask>,
    sender: Sender<Message<Job>>,
}

impl<Job> ThreadPool<Job>
where
    Job: FnOnce() + Send + 'static,
{
    pub fn new(size: usize) -> ThreadPool<Job> {
        let mut tasks = Vec::new();
        let (sender, receiver): (Sender<Message<Job>>, Receiver<Message<Job>>) = channel();
        let rec = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            tasks.push(ThreadTask::new(i, Arc::clone(&rec)));
        }
        ThreadPool { tasks, sender }
    }

    pub fn execute(&self, job: Job) {
        let _ = self.sender.send(Message::NewJob(job));
    }
}

impl<Job> Drop for ThreadPool<Job> {
    fn drop(&mut self) {
        for _ in 0..self.tasks.len() {
            let _ = self.sender.send(Message::Terminate);
        }
        for thread_task in &mut self.tasks {
            thread_task.thread.take().unwrap().join().unwrap()
        }
    }
}
