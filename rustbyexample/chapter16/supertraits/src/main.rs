trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct Entity();

impl Student for Entity{

    fn university(&self) -> String{
        String::from("zhejiang university")
    }

}

impl Person for Entity{
    fn name(&self) -> String {
        String::from("Zhang")
    }
}

impl Programmer for Entity{

    fn fav_language(&self) -> String{
        String::from("rust")
    }
}

impl CompSciStudent for Entity{

    fn git_username(&self) -> String{
        String::from("git gurudk")
    }

}

fn comp_sci_student_greeting(student: &Box<dyn CompSciStudent>) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    let student = Box::new(Entity());
    let ret = comp_sci_student_greeting(&(student as Box<dyn CompSciStudent>));
    println!("info:{}", ret);
}