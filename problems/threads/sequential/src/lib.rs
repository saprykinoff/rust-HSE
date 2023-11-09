#![forbid(unsafe_code)]

use std::marker::Send;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, Thread};

pub fn sequential_run<Printer, Iter>(
    printer: Printer,
    iterator: Iter,
    odd_thread_ms: u64,
    even_thread_ms: u64,
    max_iterations: usize,
) where
    Iter: Send + Iterator + 'static,
    Printer: Send + Fn(&Thread, Iter::Item) + 'static + std::marker::Sync,
{
    let (sender_even_ready, receiver_even_ready): (Sender<usize>, Receiver<usize>) = channel();
    let (sender_odd_ready, receiver_odd_ready): (Sender<usize>, Receiver<usize>) = channel();
    let iter_even = Arc::new(Mutex::new(iterator));
    let iter_odd = Arc::clone(&iter_even);

    let printer_even = Arc::new(Mutex::new(printer));
    let printer_odd = Arc::clone(&printer_even);

    let _ = sender_even_ready.send(max_iterations);

    let even = thread::Builder::new()
        .name("even thread".to_string())
        .spawn(move || loop {
            sleep(std::time::Duration::from_millis(even_thread_ms));
            let remains = receiver_even_ready.recv().unwrap();
            if remains == 0 {
                break;
            }
            let mut iter_guard = iter_even.lock().unwrap();
            let iter = &mut (*iter_guard);

            let printer_guard = printer_even.lock().unwrap();
            let printer = &(*printer_guard);
            let Some(nxt) = (*iter).next() else {
                break;
            };
            printer(&thread::current(), nxt);
            let _ = sender_odd_ready.send(remains - 1);
        })
        .unwrap();
    let odd = thread::Builder::new()
        .name("odd thread".to_string())
        .spawn(move || loop {
            sleep(std::time::Duration::from_millis(odd_thread_ms));
            let remains = receiver_odd_ready.recv().unwrap();
            if remains == 0 {
                break;
            }
            let mut iter_guard = iter_odd.lock().unwrap();
            let iter = &mut (*iter_guard);

            let printer_guard = printer_odd.lock().unwrap();
            let printer = &(*printer_guard);
            let Some(nxt) = (*iter).next() else {
                break;
            };
            printer(&thread::current(), nxt);
            let _ = sender_even_ready.send(remains - 1);
        })
        .unwrap();
    let _ = even.join();
    let _ = odd.join();
}
