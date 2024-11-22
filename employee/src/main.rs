use std::collections::HashMap;
fn main() {
    let mut personal = HashMap::new();

    loop {
        println!("Enter one option:");
        println!("1. Add employee");
        println!("2. Retrieve all people in a department");
        println!("3. Retrieve all people in the company");
        println!("4. Quit");
    
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        println!("You entered: {}", option);
        match option.trim().parse() {
            Ok(1) => {
                println!("Enter: Add <employee> to <department>");
                let mut addition = String::new();
                std::io::stdin().read_line(&mut addition).expect("Failed to read line");
                let tokens: Vec<&str> = addition.split(' ').collect();
                let deparment = personal.entry(tokens[3].trim().to_string()).or_insert(Vec::new());
                deparment.push(tokens[1].to_string());
                println!("Added {} to {}", tokens[1], tokens[3]);
                //println!("{:?}", personal);

            },
            Ok(2) => {
                println!("Enter deparment");
                let mut department = String::new();
                std::io::stdin().read_line(&mut department).expect("Failed to read line");
                if let Some(in_department) = personal.get(department.trim()) {
                    let mut sorted_personal = in_department.clone();
                    sorted_personal.sort();
                    println!("Employees in {} are: {:?}", department, sorted_personal);
                } else {
                    println!("No employees in {}", department);
                }
            },
            Ok(3) => {
                for (key, value) in &personal {
                    let mut sorted_personal = value.clone();
                    sorted_personal.sort();
                    println!("Department: {}, Employees: {:?}", key, sorted_personal);
                }
            },
            Ok(4) => break,        
            _ => println!("Invalid option, try again!"),
        }
    }


}
