/**
 * This is a placeholder file for borrowing and references in Rust.
 * Demonstrates one mutable or many immutable references.
 */
fn main() {
    let mut account: BankAccount = BankAccount::new(String::from("Alice"), 1000.0);
    // Immutable borrow to get balance
    account.get_balance();
    // Mutable borrow to deposit money
    account.deposit(500.0);
    // Immutable borrow to get balance
    account.get_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> Self {
        BankAccount { owner, balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Account owner {} receives ${}", self.owner, amount);
    }

    fn get_balance(&self) -> f64 {
        println!(
            "Account owner: {} has a balance of ${}",
            self.owner, self.balance
        );
        self.balance
    }
}
