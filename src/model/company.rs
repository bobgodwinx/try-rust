use crate::model::employee::Employee;
#[derive(Debug)]
pub struct Company {
    pub call_count: i64,
    pub name: String,
    pub employees: Vec<Employee>,
}

impl Company {
    pub fn count_employee(&mut self) -> usize {
        self.call_count += 1;
        return  self.employees.len();
    }
}