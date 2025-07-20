trait Account{
    fn deposit(&mut self, amount: f64) -> Result<String, String>;
    fn withdraw(&mut self, amount: f64) -> Result<String, String>;
    fn balance(&self) -> f64;
    fn account_info(&self) -> String;
}

struct BankAccount{
    account_number: String,
    holder_name: String,
    account_balance: f64
}

impl BankAccount{
    fn new(account_number: String, holder_name: String, initial_balance: f64) -> Self {
        BankAccount{
            account_number,
            holder_name,
            account_balance: initial_balance
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<String, String> {
        if amount <= 0.0 {
            Err("Insufficient transaction: amount must be greater than 0".to_string())
        } else {
            self.account_balance +=amount;
            Ok(
                format!(
                   "✅ Deposit successful! Available balance: ${:.2}",
                   self.balance()
                )
            )
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<String, String> {
        if amount <= 0.0 {
            Err("Insufficient transaction: amount must be greater than 0".to_string())
        } else if self.account_balance < amount {
            Err(format!(
                "❌ Transaction failed: insufficient funds. Available balance: ${:.2}",
                self.balance()
            ))
        } else {
            self.account_balance -=amount;
            Ok(
                format!(
                    "✅ withdraw successful!. Available balance: ${:.2}",
                    self.balance()
                )
            )
        }
    }

    fn balance(&self) -> f64 {
        return self.account_balance
    }

    fn account_info(&self) -> String {
        format!(
            "Account #{}: {} | Balance: ${:.2}",
            self.account_number, self.holder_name, self.balance()
        )
    }
}

fn main() {
    println!("🏦 Welcome to Rust Bank!");
    println!("=========================");

    let mut user0 = BankAccount::new("001".to_string(), "Youssef".to_string(), 1000.0);
    let mut user1 = BankAccount::new("002".to_string(), "Rakan".to_string(), 1000.0);
    
    println!("{}", user0.account_info());
    println!("{}", user1.account_info());

    match user0.deposit(500.0) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("❌ Error: {}", err),
    }

    match user1.withdraw(200.0) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("❌ Error: {}", err),
    }

    println!("💰 Final Balance: ${:.2}", user0.balance());
    println!("💰 Final Balance: ${:.2}", user1.balance());

    // ✅ Error test cases
    println!("\n🧪 Running error test cases...");

    let mut test_user = BankAccount::new("003".to_string(), "Tester".to_string(), 100.0);

    // ❌ Test 1: Deposit zero amount
    match test_user.deposit(0.0) {
        Ok(msg) => println!("❌ Unexpected success: {}", msg),
        Err(err) => println!("✅ Correctly caught deposit error: {}", err),
    }

    // ❌ Test 2: Deposit negative amount
    match test_user.deposit(-50.0) {
        Ok(msg) => println!("❌ Unexpected success: {}", msg),
        Err(err) => println!("✅ Correctly caught deposit error: {}", err),
    }

    // ❌ Test 3: Withdraw zero amount
    match test_user.withdraw(0.0) {
        Ok(msg) => println!("❌ Unexpected success: {}", msg),
        Err(err) => println!("✅ Correctly caught withdrawal error: {}", err),
    }

    // ❌ Test 4: Withdraw negative amount
    match test_user.withdraw(-20.0) {
        Ok(msg) => println!("❌ Unexpected success: {}", msg),
        Err(err) => println!("✅ Correctly caught withdrawal error: {}", err),
    }

    // ❌ Test 5: Withdraw more than available balance
    match test_user.withdraw(1000.0) {
        Ok(msg) => println!("❌ Unexpected success: {}", msg),
        Err(err) => println!("✅ Correctly caught insufficient funds: {}", err),
    }

    println!("✅ Final balance remains: ${:.2}", test_user.balance());
}

