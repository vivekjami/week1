// Define the Task struct
#[derive(Clone, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// Create a vector to store instances of the Task struct
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    // Implement the add_task function
    fn add_task(&mut self, description: &str) -> Task {
        let task_id = self.tasks.len() + 1;
        let new_task = Task {
            id: task_id,
            description: String::from(description),
            completed: false,
        };
        self.tasks.push(new_task.clone());
        new_task
    }

    // Implement the complete_task function
    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    // Implement the list_tasks function
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    // Create an instance of TodoList
    let mut todo_list = TodoList { tasks: Vec::new() };

    // Add tasks
    let task1 = todo_list.add_task("Buy groceries");
    let task2 = todo_list.add_task("Read a book");

    // List tasks
    println!("Tasks before completion:");
    todo_list.list_tasks();

    // Complete a task
    todo_list.complete_task(task1.id);

    // List tasks after completion
    println!("Tasks after completion:");
    todo_list.list_tasks();
}
