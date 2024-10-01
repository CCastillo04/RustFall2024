use std::io::{self,Read, Write};
use std::fs::File;

struct Car {
    model: String,
    year: u32,
}

fn readconsole_tofile() {
    let mut file = File::create("user_info.txt").unwrap();

    let mut new_line = String::new();

    print!("Whats your car model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_line).unwrap();
    let model = new_line.trim().to_string();
    new_line.clear();

    print!("What year is it?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_line).unwrap();
    let year: u32 = new_line.trim().parse().unwrap();

    let _car = Car {model, year};

    writeln!(file,"{}", _car.model).unwrap();
    writeln!(file,"{}", _car.year).unwrap();

}

impl Car{

    fn from_file(_path: &str) -> Car {
        let mut file = File::open("user_info.txt").unwrap();
        let mut new_line = String::new();

        file.read_to_string(&mut new_line).unwrap();

        let mut lines = new_line.lines();

        let model = lines.next().unwrap().to_string();
        let year = lines.next().unwrap().parse().unwrap();

      Car { model, year }
    }
}

fn readfile(){
    let car = Car::from_file("user_info.txt");
    println!("Model: {}",car.model);
    println!("Year: {}", car.year)
}

fn main(){
    readconsole_tofile();
    readfile();
}