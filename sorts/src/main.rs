use clap::{App, Arg};
// use std::fs::File;
// use std::io::{prelude::*, BufReader, BufWriter};

/*
Implement the bubble sort, shaker sort, and counting sort as Python functions.
Put it all together inside a main function.
Also, test to make sure all your sorting functions are working.
Do this by copying the original list, useing the built in Python sort method on the copy, then checking if your sorted list matches the sorted copy.
Do this multiple times to verify that each of your sorting algorithms are working.
*/


mod unsorted;
mod bubble;
mod shaker;

fn main() {
    let matches = cli();

    // get the files
    // let in_path = matches.value_of("input").unwrap(); // unwrap OK because required arg
    // let out_path = matches.value_of("output").unwrap(); // unwrap OK because required arg

    // get the size
    let size_of_list = matches.value_of("size").unwrap(); // unwrap OK because of default

    let size: usize = size_of_list.parse::<i32>().unwrap() as usize;

    // bufreading
    // let f = File::open(in_path).expect("Unable to open file");
    // let f = BufReader::new(f);

    let range: i32 = 50;

    let unsorted: Vec<i32> = unsorted::create(size, range);
    let unsorted_str: String = unsorted.clone().into_iter().map(|i| i.to_string() + " ").collect::<String>();
    println!("Unsorted: {}", unsorted_str);

    // call selected sorting algorithm
    let b_sorted: Vec<i32> = bubble::sort(unsorted.clone());
    let b_sorted_str: String = b_sorted.into_iter().map(|i| i.to_string() + " ").collect::<String>();

    let s_sorted: Vec<i32> = shaker::sort(unsorted);
    let s_sorted_str: String = s_sorted.into_iter().map(|i| i.to_string() + " ").collect::<String>();

    println!("B Sorted: {}", b_sorted_str);
    println!("S Sorted: {}", s_sorted_str);

    // writing
    // let out_f = File::create(out_path).expect("Couldn't create file!");
    // let mut out_f = BufWriter::new(out_f);
    // out_f
    //     .write(unsorted_str.as_bytes())
    //     .expect("Couldn't write contents out!");
}

fn cli() -> clap::ArgMatches {
    let matches = App::new("Sorts Collection")
    .args(&[
        // Arg::new("input")
        //     .required(true)
        //     .index(1)
        //     .help("the input file to sort"),
        // Arg::new("output")
        //     .required(true)
        //     .index(2)
        //     .help("the output that holds sorted data"),
        Arg::new("size")
            .short('s')
            .default_value("10")
            .help("the size of the list to be sorted(default=10)"),
        Arg::new("alg")
            .short('a')
            .help("the type of algorithm you want to use"),
        Arg::new("range")
            .short('r')
            .default_value("20")
            .help("the range numbers are allowed")
    ])
    .get_matches();
return matches;
}