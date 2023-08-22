use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time::Duration};

pub fn run() {
    // simple_threads();

    // waiting_thread();

    // move_closure();

    // channel();

    // shared_state();

    // incrementable_thread();
}

fn simple_threads() {
    // all rust threads are 1:1 with OS threads, no Green Threads by the standard lib
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread, as soon as finishes, all threads are killed
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn waiting_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait the spawned thread to finish, main prevented from other work
    handle.join().unwrap();
}

fn move_closure() {
    let v = vec![1, 2, 3];

    // move closure, takes ownership of v
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // v is no longer usable in main thread
    // drop(v); // error: value used here after move

    handle.join().unwrap();
}

fn channel() {
    // mpsc: multiple producer, single consumer
    // threads communicate
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // clone tx to send from multiple threads

    // move tx to thread
    thread::spawn(move || {
        let msg = String::from("hi");

        // send message, if receiver is dropped -> error
        tx.send(msg).unwrap();
        // println!("val is {}", val); // error: send takes ownership of msg
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // send multiple messages
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // recv blocks main thread execution, if channel is closed -> error
    // try_recv is non-blocking, checks if there is new message
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // multiple messages
    for received in rx {
        println!("Got: {}", received);
    }
}

fn shared_state() {
    // mutural exclusion mutex (lock on data)
    let m = Mutex::new(5);

    {
        // block the current thread until acquasition of lock
        // if other thread panics -> mutex is poisoned
        let mut num = m.lock().unwrap();

        // deref smart pointer
        *num = 6;

        // auto lock release by smart pointer
    }

    println!("m = {:?}", m);
}

fn incrementable_thread() {
    // atomic reference smart pointer, thread safe
    let counter = Arc::new(Mutex::new(0));

    // create threads
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        // clone counter for each thread
        let handle = thread::spawn(move || {
            // lock counter
            let mut num = counter.lock().unwrap();

            // increment
            *num += 1;
        });

        // store thread handles
        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
