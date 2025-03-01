use std::sync::{Arc, Mutex};
use std::thread;
//Used to create delays and timeouts
use std::time::{Duration, Instant};

// Define a struct for a bank account
struct Account {
    balance: i32,
    id: usize, // Unique account ID
}

impl Account {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
        println!("Account {} Deposited: ${}, New Balance: ${}", self.id, amount, self.balance);
    }

    fn withdraw(&mut self, amount: i32) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Account {} Withdrew: ${}, New Balance: ${}", self.id, amount, self.balance);
            return true;
        } else {
            println!("Account {} Failed Withdrawal: Not enough funds. Current Balance: ${}", self.id, self.balance);
            return false;
        }
    }
}

// Function to transfer money between accounts
fn transfer(from: &mut Account, to: &mut Account, amount: i32) {
    if from.withdraw(amount) {
        to.deposit(amount);
        println!("Transferred ${} from Account {} to Account {}", amount, from.id, to.id);
    } else {
        println!("Transfer Failed: Account {} has insufficient funds.", from.id);
    }
}

fn main() {
    let num_accounts = 2; // Two accounts to demonstrate deadlock
    let accounts: Arc<Vec<Mutex<Account>>> = Arc::new(
        (0..num_accounts)
            .map(|id| Mutex::new(Account { balance: 1000, id }))
            .collect(),
    );

    let mut handles = vec![];

    // Create a deadlock scenario: Two threads trying to transfer money at the same time
    for i in 0..2 {
        let accounts_clone = Arc::clone(&accounts);
        let handle = thread::spawn(move || {
            let start_time = Instant::now();
            let from_id = i; // Each thread has a different "from" account
            let to_id = 1 - i; // The opposite account is the destination
            let amount = 100;
            let timeout = Duration::from_secs(9);
            // **Deadlock Risk: Locks are acquired in different orders by different threads**
            let mut from_account = accounts_clone[from_id].lock().unwrap();
            println!("Thread {} locked Account {}", i, from_id);
            thread::sleep(Duration::from_millis(100)); // Simulating delay
            loop {
                if let Ok(mut to_account) = accounts_clone[to_id].try_lock() {
                    println!("Thread {} locked Account {}", i, to_id);
                    transfer(&mut *from_account, &mut *to_account, amount);
                    break;
                }
                if start_time.elapsed() > timeout {
                    println!("Warning dead lock detected in Thread {}", i);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete (this will get stuck due to deadlock)
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balances (never reached in case of deadlock)
    println!("\nFinal Account Balances:");
    for account in accounts.iter() {
        let acc = account.lock().unwrap();
        println!("Account {}: ${}", acc.id, acc.balance);
    }
}