use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter command (Add, Retrieve, or Exit):");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "Add" => add_employee(&mut company),
            "Retrieve" => retrieve_employees(&company),
            "Exit" => break,
            _ => println!("Invalid command"),
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    println!("Enter 'Employee Name' to 'Department':");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() == 3 && parts[1].to_lowercase() == "to" {
        let department = parts[2].to_string();
        let employee = parts[0].to_string();
        company
            .entry(department)
            .or_insert(Vec::new())
            .push(employee);
        println!("Employee added.");
    } else {
        println!("Invalid format. Use 'Add Employee to Department'.");
    }
}

fn retrieve_employees(company: &HashMap<String, Vec<String>>) {
    for (department, employees) in company {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("{}: {:?}", department, sorted_employees);
    }
}
