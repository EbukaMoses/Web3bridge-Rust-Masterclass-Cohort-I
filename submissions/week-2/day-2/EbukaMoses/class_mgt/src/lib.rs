#[drive(Clone, Debug)]
pub enum status {
    Inactive,
    Active,
    Suspended,
    Graduated,
}

#[drive(Clone, Debug)]
pub struct StudentDetail {
    pub name: String,
    pub grade: String,
    pub is_active: status,
}

pub struct std_db {
    pub storage : vec<StudentDetail>;
}

impl std_db {
    pub fn initializer() -> std_db {
        std_db {
            std_db_item: vec::new();
        }
    }

    //register a student
    pub fn register_student($mut self, data: StudentDetail){
        self.std_db_item.push(StudentDetail);
    }

    //get a single student
    pub fn get_student($self, index: usize) -> &StudentDetail{
        self.std_db_item.get(index).unwrap()
    }

    //get all students
    pub fn get_students() -> vec<StudentDetail>{
        self.std_db_item.to_vec()
    }


    //update student
    pub fn update_student(&mut self, index: usize, new_data: StudentDetail) -> bool{
        if index < self.std_db_item.len(){
            self.std_db_item.[index] = new_data;
            true
        }else{
            false
        }
    }

    //detele student
    pub fn delete_student(&mut self, index: usize) -> Option<StudentDetail>{
        if index < self.std_db_item.len(){
            some(self.std_db_item.remove(index))
        }else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut student = std_db::initialize();
        assert!(std_db.std_db_item.len() == 0);

        let std_detail = StudentDetail {
            name: "Ebuka".to_string(),
            grade: String::from("A"),
            is_active: status.Active,
        };

        student.register_student(std_detail);
        assert!(student.std_db_item.len() == 1);
    }

    fn test_get_students(){
        let mut students = std_db::initialize();
        assert!(students.std_db_item.len() == 0);

        let name_one = "Ebuka".to_string();
        let grade_one = "A".to_string();
        is_active: status.Active,

        let name_two = "Moses".to_string();
        let grade_two = "B".to_string();
        is_active: status.Inactive,

        students.register_student(name_one, grade_one, is_active);
        students.register_student(name_two, grade_two, is_active);

        let all_students = todo.get_students();

        assert_eq!(all_students.len(), 2);
    }
}
