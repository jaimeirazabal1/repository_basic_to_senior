use std::io::{self, Write};

struct TodoItem {
    description: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("\nTodo List Manager");
        println!("----------------");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Mark task as completed");
        println!("4. Remove task");
        println!("5. Exit");

        match get_choice() {
            1 => add_task(&mut todo_list),
            2 => list_tasks(&todo_list),
            3 => mark_completed(&mut todo_list),
            4 => remove_task(&mut todo_list),
            5 => break,
            _ => println!("Invalid choice! Please select 1-5"),
        }
    }
}

fn get_choice() -> i32 {
    print!("Enter your choice (1-5): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn add_task(todo_list: &mut Vec<TodoItem>) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    todo_list.push(TodoItem {
        description: input.trim().to_string(),
        completed: false,
    });
    println!("Task added successfully!");
}

fn list_tasks(todo_list: &Vec<TodoItem>) {
    if todo_list.is_empty() {
        println!("No tasks in the list!");
        return;
    }

    println!("\nCurrent Tasks:");
    for (index, task) in todo_list.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}. [{}] {}", index + 1, status, task.description);
    }
}

fn mark_completed(todo_list: &mut Vec<TodoItem>) {
    list_tasks(todo_list);
    if todo_list.is_empty() {
        return;
    }

    print!("Enter task number to mark as completed: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= todo_list.len() {
            todo_list[index - 1].completed = true;
            println!("Task marked as completed!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input!");
    }
}

fn remove_task(todo_list: &mut Vec<TodoItem>) {
    list_tasks(todo_list);
    if todo_list.is_empty() {
        return;
    }

    print!("Enter task number to remove: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= todo_list.len() {
            todo_list.remove(index - 1);
            println!("Task removed successfully!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input!");
    }
}