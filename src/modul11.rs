#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//Threads
//Channels
//Mutex

//-----------------------------------------Threads
//Run code in parallel
// Ownership/borrowing mechanism gives us
// - memory safety
// - no data races

//Create a thread
//use std::thread;
//let th = thread::spawn(closure)
//Sleep a thread
//th::sleep()
//Wait for a thread
//th.join()

use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn mod11_threads() {
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 1000));
            println!("new thread {}", i);
        });
        threads.push(th);
    }

    for t in threads {
        t.join();
    }

    println!("Main thread");
}

//-----------------------------------------Channels
//A way to send data between threads
//MPSC - multiple producer single receiver
//Create a channel
//use std::sync::mpsc;
//let (tx, rx) = mpsc::channel();
//Send a message
//tx.send()

//Receive a message
//rx.recv()		// blocking
//rx.try_recv()	// non blocking

use std::sync::mpsc;

const NUM_THREDS: usize = 20;

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("setting timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("sending {}", d);
        tx.send(d).unwrap()
    });
}

pub fn mod11_chan() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || { tx.send(42).unwrap() });
    // println!("received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREDS {
        start_thread(i, tx.clone());
    }

    for j in rx.iter().take(NUM_THREDS) {
        println!("received {}", j);
    }
}

//-----------------------------------------Mutex
//Mutual exclusion lock
//Only one thread can access the data at any one time
//Arc - atomically referenced counted type convert data into primitive types, safe to share across threads
//Create a lock
//use std::sync::{Mutex, Arc};
//let lock = Arc::new(Mutex::new(0));

//Acquire a lock
//lock.lock()
//lock.try_lock()
// Poisoned lock - when a thread that holds the lock panics
//lock.is_poisoned()

use std::sync::{Arc, Mutex};

pub fn mod11_mutex() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(t);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Result {}", *c.lock().unwrap());
}
