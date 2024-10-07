#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(self: &Self) {
        //TODO! (implement printing info about Student
         // use match statement)

       match &self.grade{
           GradeLevel::Bachelor => println!("{} is a Bachelor student", self.name),
           GradeLevel::Master => println!("{} is a Master student", self.name),
           GradeLevel::PhD => println!("{} is a PhD student",self.name),
       }

       match &self.major{
        Major::ComputerScience => println!("{} is also a CS student.", self.name),
        Major::ElectricalEngineering => println!("{} is also an Electrical Engin", self.name),
    }
        }
}


fn main() {
    let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();
}