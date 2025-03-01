//ARC and AtomicI32 used to create data that is shared safely between threads
use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
//Standard threading rust threading library
use std::thread;
//Used to create delays
use std::time::Duration;

fn main() {
    // Balance represents the entire bank
    // Balance is shared by multiple threads so wrapped in ARC, atomicI32 number to allow multiple threads access
    let balance = Arc::new(AtomicI32::new(1000));
    //Vector to store all the created threads
    let mut handles = vec![];

    // Spawn 5 separate deposit threads
    for i in 0..5 {
        let balance_clone = Arc::clone(&balance);
        let handle = thread::spawn(move || {
            //edit amount that can be added to the balance
            let amount = 100;
            //Used to simulate different delays and test concurrency
            //Delay for deposits
            thread::sleep(Duration::from_millis(50));
            balance_clone.fetch_add(amount, Ordering::SeqCst);
            println!("Thread {} Deposited: ${}, New Balance: ${}", i, amount, balance_clone.load(Ordering::SeqCst));
        });
        handles.push(handle);
    }

    // Spawn 5 separate withdrawal threads
    for i in 0..5 {
        let balance_clone = Arc::clone(&balance);
        let handle = thread::spawn(move || {
            let amount = 300;
            //Set delay for withdrawal threads
            thread::sleep(Duration::from_millis(50));
            let current_balance = balance_clone.load(Ordering::SeqCst);
            if current_balance >= amount {
                balance_clone.fetch_sub(amount, Ordering::SeqCst);
                println!("Thread {} Withdrew: ${}, New Balance: ${}", i, amount, balance_clone.load(Ordering::SeqCst));
            } else {
                println!("Thread {} Failed Withdrawal: Not enough funds. Current Balance: ${}", i, current_balance);
            }
            thread::sleep(Duration::from_millis(50)); // Simulate processing time
        });
        //Pushes threads to the vector
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balance
    println!("Final Balance: ${}", balance.load(Ordering::SeqCst));
}