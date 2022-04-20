use crate::model::employee::Employee;
#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub employees: Vec<Employee>,
}