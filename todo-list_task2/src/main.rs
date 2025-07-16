use std::io::{self, Write};

#[derive(Debug)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    priority: Priority,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String, priority: Priority) -> Task {
        Task {
            id,
            description,
            priority,
            completed: false,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut task_id_counter = 1;

    println!("🛠️ Welcome to CLI To-Do List !");
    println!("===========================================");

    loop {
        println!("+-----------------------------------------+");
        println!("|         📝 To-Do List Manager           |");
        println!("+-----------------------------------------+");
        println!("| 1. ✅ Add Task                          |");
        println!("| 2. 📋 List Tasks                        |");
        println!("| 3. ✔️ Mark Task as Completed             |");
        println!("| 4. ❌ Delete Task                       |");
        println!("| 5. 👋 Exit                              |");
        println!("+-----------------------------------------+");

        let choice = read_input("👉 Enter your choice (1-5): ");

        match choice.trim() {
            "1" => add_task(&mut tasks, &mut task_id_counter),
            "2" => fetch_tasks(&tasks),
            "3" => mark_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("👋 Thank you for using To-Do List Manager!");
                break;
            }
            _ => println!("⚠️ Invalid choice. Please enter 1-5."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>, task_id_counter: &mut usize) {
    println!("\n--- Adding New Task ---");

    let description = read_input("Enter task description: ");
    if description.is_empty() {
        println!("❌ Task description cannot be empty!");
        return;
    }

    let priority_input = read_input("Enter priority (High/Medium/Low): ");
    let priority = match priority_input.trim().to_lowercase().as_str() {
        "high" | "h" => Priority::High,
        "medium" | "m" => Priority::Medium,
        "low" | "l" => Priority::Low,
        _ => {
            println!("❌ Invalid priority! Task not added.");
            return;
        }
    };

    let task = Task::new(*task_id_counter, description, priority);
    tasks.push(task);
    println!("✅ Task added successfully with ID {}!", task_id_counter);
    *task_id_counter += 1;
}

fn fetch_tasks(tasks: &Vec<Task>) {
    println!("\n--- Your Tasks ---");
    check_task(tasks);

    let completed_count = tasks.iter().filter(|t| t.completed).count();
    println!(
        "Completed: {} | Pending: {}",
        completed_count,
        tasks.len() - completed_count
    );
    println!("---");
}

fn mark_completed(tasks: &mut Vec<Task>) {
    println!("\n--- Mark Task as Completed ---");

    if tasks.is_empty() {
        println!("📭 No tasks to mark as completed!");
        return;
    }

    let pending_tasks: Vec<&Task> = tasks.iter().filter(|t| !t.completed).collect();
    if pending_tasks.is_empty() {
        println!("🎉 All tasks are already completed!");
        return;
    }

    let input_id = read_input("Enter Task ID to mark as completed: ");
    match input_id.trim().parse::<usize>() {
        Ok(id) => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                if task.completed {
                    println!("⚠️ Task with ID {} is already completed.", id);
                } else {
                    task.completed = true;
                    println!("✅ Task {} '{}' marked as completed!", id, task.description);
                }
            } else {
                println!("⚠️ Task with ID {} not found.", id);
            }
        }
        Err(_) => println!("❌ Invalid task ID."),
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("\n--- Delete Task ---");

    check_task(tasks);

    let id_input = read_input("Enter Task ID to delete: ");
    match id_input.trim().parse::<usize>() {
        Ok(id) => {
            if let Some(index) = tasks.iter().position(|t| t.id == id) {
                let deleted_task = tasks.remove(index);
                println!(
                    "🗑️ Task {} '{}' deleted successfully!",
                    id, deleted_task.description
                );
            } else {
                println!("⚠️ Task with ID {} not found.", id);
            }
        }
        Err(_) => println!("❌ Invalid task ID. Please enter a valid number."),
    }
}

fn check_task(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("📭 No tasks to display!");
        return;
    }

    for task in tasks {
        let status = if task.completed { "✅ Done" } else { "❌ Pending" };
        let priority = match task.priority {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        println!(
            "[{}] {} | Priority: {} | Status: {}",
            task.id, task.description, priority, status
        );
    }
}
