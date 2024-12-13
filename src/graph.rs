

use crate::employee::Employee;
use std::collections::HashMap;
#[allow(dead_code)]
pub struct EmployeeGraph { //create struct of graph
    pub employees: Vec<Employee>,
    pub connections: HashMap<u32, Vec<u32>>,
}

impl EmployeeGraph { //implement graph
    pub fn new() -> Self {
        EmployeeGraph { //create vector and hashmap
            employees: Vec::new(),
            connections: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, employee: &Employee) {
        self.employees.push(employee.clone()); //assume clone
    }
}
