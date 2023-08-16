// Define Student struct
struct Student {
    id: u32,
    name: String,
}

fn create_student(id: u32, name: String) -> Student {
    Student { id, name }
}

fn read_student(student: &Student) {
    println!("Student ID: {}, Name: {}", student.id, student.name);
}

fn update_student(student: &mut Student, new_name: String) {
    student.name = new_name;
}

fn delete_student(students: &mut Vec<Student>, student_id: u32) {
    students.retain(|s| s.id != student_id);
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    let student1 = create_student(1, String::from("Alice"));
    let student2 = create_student(2, String::from("Bob"));

    students.push(student1);
    students.push(student2);

    for student in &students {
        read_student(student);
    }

    if let Some(mut student) = students.iter_mut().find(|s| s.id == 1) {
        update_student(&mut student, String::from("Alicia"));
    }

    delete_student(&mut students, 2);

    println!("Remaining students:");
    for student in &students {
        read_student(student);
    }
}