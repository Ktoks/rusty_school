use std::collections::HashMap;
use text_io::read;

fn main() {
    // let matches = cli();
    println!("Welcome to ISBN Indexer!");
    let mut dict_of_books = HashMap::new();

    let mut quitting = false;

    while !quitting {
        // greeting
        println!("What would you like to do?\nInput one of the following: \nr to record a book\nf to find a book\nl to list all books\nq or anything else to quit");
        // get input from user
        let decision: String = read!("{}\n");
       
        // record book
        if "r" == decision {

            println!("Please input the book isbn: ");
            let isbn: String = read!("{}\n");

            println!("Please input the book title: ");
            let book_title: String = read!("{}\n");

            // insert isbn:title into hashmap
            dict_of_books.insert(isbn, book_title);
        }
        // find book
        else if "f" == decision {

            println!("\nPlease input the isbn of the book you're looking for: ");
            let isbn: String = read!("{}\n");

            // show book found
            println!("\nHere is your book: {}\n", dict_of_books.get(&isbn).unwrap());
        }
        // list all books in hashmap
        else if "l" == decision {
            if dict_of_books.len() < 1 {
                println!("\nThere are no books in here yet!\n");
            } else{
                for (key, value) in dict_of_books.iter() {
                    println!("ISBN: {:?} Name: {:?}\n", key, value);
                }
            }
        }
        // quit if 'q' is input
        else if "q" == decision {
            println!("\nThanks for using ISBN Index!");
            quitting = true;
        }
        // quit if invalid input is recieved
        else {
            println!("\nNot an expected input, quitting...");
            quitting = true;
        }
    }
}
