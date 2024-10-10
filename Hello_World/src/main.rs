use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Write};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    //creates file
    let mut file = File::create(filename).unwrap();

    //writes into file with '|' between the words
    for book in books{
        writeln!(file,"{}|{}|{}", book.title, book.author, book.year);
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    //creates new vector
    let mut books: Vec<Book> = Vec::new();

    //for every line in the file it will
    for line in reader.lines() {
        
        //unwrap it
        let line = line.unwrap();
        //split at '|' and saving the words into a vector called parts
        let parts: Vec<&str> = line.split('|').collect();

        //when parts has the length of 3
        if parts.len() == 3 {

            //it saves the element in parts into book struct
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2].parse().unwrap();

            //pushes it into the vector made
            books.push(Book { title, author, year });
        }
    }
    // returns books
    books
}



fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}