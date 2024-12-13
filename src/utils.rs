//make readable

use crate::graph::EmployeeGraph;
use crate::employee::Employee;
use csv::ReaderBuilder;

pub fn load_employee_data(file_path: &str) -> EmployeeGraph {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path).expect("Failed to open file"); //error handling
    let mut graph = EmployeeGraph::new();
    let mut employees: Vec<Employee> = Vec::new();

    for result in rdr.records() { //making sure it is readable
        if let Ok(record) = result {
            let employee = Employee { //converts to correct type to avoid errors
                id: record[0].parse().unwrap(),
                name: record[1].to_string(),
                department: record[4].to_string(),
                performance_score: if record[7].is_empty() { None } else { Some(record[7].parse().unwrap()) },
                age: record[2].parse().unwrap(),
                gender: record[3].to_string(),
                salary: record[5].parse().unwrap(),
                joining_date: record[6].to_string(),
                experience: record[8].parse().unwrap(),
                status: record[9].to_string(),
                location: record[10].to_string(),
                session: record[11].to_string(),
            };
            employees.push(employee);
        }
    }

    // Add employees as references\
    for employee in employees.iter() {
        graph.add_employee(employee);
    }

    graph
}


