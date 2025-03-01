use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng; // Import rand for random number generation

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

// Function to transfer money between accounts safely
fn transfer(from: &mut Account, to: &mut Account, amount: i32) {
    if from.withdraw(amount) {
        to.deposit(amount);
        println!("Transferred ${} from Account {} to Account {}", amount, from.id, to.id);
    } else {
        println!("Transfer Failed: Account {} has insufficient funds.", from.id);
    }
}

fn main() {
    let start_time = Instant::now();
    let num_accounts = 5; // Create 5 accounts
    let accounts: Arc<Vec<Mutex<Account>>> = Arc::new(
        (0..num_accounts)
            .map(|id| Mutex::new(Account { balance: 1000, id }))
            .collect(),
    );

    let mut handles = vec![];
    let mut rng = rand::thread_rng(); // Create a random number generator

    // Spawn multiple threads for transactions
    for _ in 0..10 { // 10 transactions happening concurrently
        let accounts_clone = Arc::clone(&accounts);
        let from_id = rng.gen_range(0..num_accounts);
        let mut to_id = rng.gen_range(0..num_accounts);
        while to_id == from_id {
            to_id = rng.gen_range(0..num_accounts);
        }

        let handle = thread::spawn(move || {
            let amount = 100;

            // **Fix: Always lock accounts in order of their IDs**
            let (first, second) = if from_id < to_id {
                (&accounts_clone[from_id], &accounts_clone[to_id])
            } else {
                (&accounts_clone[to_id], &accounts_clone[from_id])
            };

            // Lock both accounts in order
            let mut first_account = first.lock().unwrap();
            println!("Thread {:?} locked Account {}", thread::current().id(), first_account.id);
            thread::sleep(Duration::from_millis(50)); // Simulating delay

            let mut second_account = second.lock().unwrap();
            println!("Thread {:?} locked Account {}", thread::current().id(), second_account.id);

            // Perform the transfer
            if first_account.id == from_id {
                transfer(&mut first_account, &mut second_account, amount);
            } else {
                transfer(&mut second_account, &mut first_account, amount);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balances
    println!("\nFinal Account Balances:");
    for account in accounts.iter() {
        let acc = account.lock().unwrap();
        println!("Account {}: ${}", acc.id, acc.balance);
    }
    println!("Run time {}ms", start_time.elapsed().as_millis());
}
