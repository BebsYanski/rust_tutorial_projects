fn main() {
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.34,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraww(50.35);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Method to withdraw money
    fn withdraww(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }
    // Method to check the balance
    fn check_balance(&self) {
        println!(
            "Account owned by {}, and has a balance of {:.2}",
            self.owner, self.balance
        )
    }
}
