//Add Mutex to allow the locking of shared resources
use std::sync::{Arc, Mutex};
//Standard threading rust threading library
use std::thread;
//Used to create delays
use std::time::Duration;

//Create A Struct for accounts
struct Account {
    balance: i32,
    number_transactions: i32,
}
impl Account {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
        self.number_transactions += 1;
        println!("Deposited: ${}, New Balance: ${}", amount, self.balance);
    }
    fn withdraw(&mut self, amount: i32) {
        self.number_transactions += 1;
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew: ${}, New Balance: ${}", amount, self.balance);
        } else {
            println!("Failed Withdrawal: Not enough funds. Current Balance: ${}", self.balance);
        }
    }
}
fn main() {
    //Create an account that is wrapped in ARC and Mutex for shared + thread safe access
    let account = Arc::new(Mutex::new(Account { balance: 1000, number_transactions: 0 }));
    //Vector to store all the created
    let mut handles = vec![];

    // Spawn 5 separate deposit threads
    for _ in 0..5 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            //locks the account for use by this thread
            let mut acc = account_clone.lock().unwrap();
            let amount = 200;
            //Set delay for deposit threads
            thread::sleep(Duration::from_millis(50));
            acc.deposit(amount);
        });
        handles.push(handle);
    }

    // Spawn 5 separate withdrawal threads
    for _ in 0..5 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            //locks the account for use by this thread
            let mut acc = account_clone.lock().unwrap();
            let amount = 300;
            //Set delay for withdrawal threads
            thread::sleep(Duration::from_millis(50));
            acc.withdraw(amount)
        });
        //Pushes threads to the vector
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balance
    let final_balance = account.lock().unwrap().balance;
    let num_transactions = account.lock().unwrap().number_transactions;
    println!("Final Balance: ${}", final_balance);
    println!("Total transactions: {}", num_transactions);
}