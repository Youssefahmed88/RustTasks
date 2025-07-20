# 🮀 Rust Learning Tasks

This repository contains small Rust projects to practice and reinforce core Rust programming concepts, such as ownership, structs, enums, method syntax, and CLI-based project design.

---

## ✅ Task 1: Ownership Basics

📁 Folder: `ownership_task`

* Focus: Understanding how Rust handles **ownership**, **borrowing**, and **string slices**.
* Implemented a function `concatenate_strings` that takes two `&str` references and returns an owned `String`.
* Avoided ownership violations by using references correctly.

### 🔍 Key Concepts:

* `String` vs `&str`
* Borrowing and references
* Returning owned data

### 🧰 Explanation:

* `concatenate_strings()`:

  * Takes two `&str` references to avoid taking ownership.
  * Creates a new `String` from `str1` and appends `str2`.
  * Returns the newly created owned `String`.
* In `main()`:

  * Allocates `String` variables on the heap.
  * Passes them as references to avoid moving them.
  * Receives and prints the concatenated result.

---

## ✅ Task 2: CLI To-Do List Manager

📁 Folder: `todo_cli`

* Built a full-featured **Command-Line To-Do List Manager** in Rust.
* Users can interactively add, list, mark, and delete tasks via terminal.

### 🌟 Features:

* Add task with description and priority (`High`, `Medium`, or `Low`)
* List all tasks with completion status
* Mark a task as completed
* Delete a task by ID
* Dynamic task ID auto-increment
* User-friendly terminal prompts and error handling

### 💡 Concepts Practiced:

* Structs and enums
* Method syntax using `impl`
* Mutable references and ownership
* Vectors and iteration
* Pattern matching (`match`)
* Terminal I/O with `std::io`

### 🖥️ Sample Interaction:

```
== To-Do List Manager ==
1. ✅ Add Task
2. 📋 List Tasks
3. ✔️ Mark Task as Completed
4. ❌ Delete Task
5. 👋 Exit

👉 Enter your choice: 1
Enter task description: Study Rust
Enter priority (High/Medium/Low): High
✅ Task added successfully with ID 1!
```
## 🧠 Task 3 - Banking System Using Traits

### ✅ Objective:

Implement a simple banking system using Rust’s `trait` and `struct` to simulate basic operations like deposits, withdrawals, and viewing account info.

---

### 🛡️ Features:

- Define a trait `Account` with the following methods:
  - `deposit(amount: f64) -> Result<String, String>`
  - `withdraw(amount: f64) -> Result<String, String>`
  - `balance() -> f64`
  - `account_info() -> String`

- Create a struct `BankAccount` with:
  - `account_number: String`
  - `holder_name: String`
  - `account_balance: f64`

- Implement the `Account` trait for `BankAccount`.

---

### 📦 Sample Output:

```text
🏦 Welcome to Rust Bank!
=========================
Account #001: Youssef | Balance: $1000.00
Account #002: Rakan | Balance: $1000.00
✅ Deposit of $500.00 successful.
✅ Withdrawal of $200.00 successful.
💰 Final Balance: $1500.00
💰 Final Balance: $800.00
```

---

### ❌ Error Test Cases:

#### ❌ Deposit Negative Amount:
```text
❌ Error: ❌ Transaction failed: amount must be greater than 0
```

#### ❌ Withdraw Negative Amount:
```text
❌ Error: ❌ Transaction failed: amount must be greater than 0
```

#### ❌ Withdraw More Than Balance:
```text
❌ Error: ❌ Transaction failed: insufficient funds. Available balance: $800.00

---

## 🚀 How to Run

Make sure you have Rust installed:

```bash
rustup show
```

Then go to each task's folder and run:

```bash
cargo run
```

---

## 📚 Useful Resources

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)