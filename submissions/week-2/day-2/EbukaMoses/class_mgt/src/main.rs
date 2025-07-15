pub mod lib;
use crate::lib::{Todo, TodoItem};

fn main() {

    let mut students = std_db::initialize();

    let name_one = "Ebuka".to_string();
    let grade_one = "A".to_string();
    is_active: status.Active,

    let name_two = "Moses".to_string();
    let grade_two = "B".to_string();
    is_active: status.Inactive,

    students.register_student(name_one, grade_one, is_active);
    students.register_student(name_two, grade_two, is_active);

    let all_students = todo.get_students();

    println!("Todos are {:#?}", all_studentslen(), 2);
}