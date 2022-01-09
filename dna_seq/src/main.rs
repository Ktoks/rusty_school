use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // cli
    let matches = App::new("XML Format")
        .args(&[
            Arg::with_name("input")
                .required(true)
                .index(1)
                .help("the input file to use"),
            Arg::with_name("output")
                .short("o")
                .required(true)
                .takes_value(true)
                .help("the output file to use"),
        ])
        .get_matches();

    let target_f = matches.value_of("input").unwrap();
    let candidates_f = matches.value_of("output").unwrap();
    // println!("in path: {:?}\nout path: {:?}", target_f, candidates_f);

    // get target
    let target_lst = file_to_list(target_f);

    // get candidates
    let cans_lst = file_to_list(candidates_f);

    let mut best_can: std::string::String = String::from("");

    let mut largest_overlap_can: usize = 0;

    let tar_bytes = target_lst[0].as_bytes();
    if strand_is_empty(tar_bytes) {
        println!("empty target strand");
    }

    for candidate in cans_lst {
        // println!("Can: {}", candidate);
        let (overlap, candi): (usize, std::string::String) =
            find_largest_overlap(tar_bytes, candidate);
        if overlap > largest_overlap_can {
            largest_overlap_can = overlap;
            best_can = candi;
        }
    }
    // let tarz = tar_bytes.to_owned();
    let tary = std::string::String::from_utf8_lossy(tar_bytes);
    let tarz = tary.into_owned();
    println!("The string is: {}", join_two_strands(tarz, best_can, largest_overlap_can))
}

fn join_two_strands(tar: std::string::String, can: std::string::String, overlap: usize) -> std::string::String {
    let newstr = &tar[0..(tar.len() - overlap)];
    return [String::from(newstr), can].concat();
}

fn find_largest_overlap(tar: &[u8], can: std::string::String) -> (usize, std::string::String) {
    let largest = 0;
    let newcan = can.as_bytes();
    let tarlen = tar.len();
    for i in 1..tarlen - 1 {
        // todo testing
        // println!("tar: {:?}\ncan: {:?}", &tar[i-1..tarlen], &newcan[0..i]);
        if candidate_overlaps_target(&tar[tarlen - i..tarlen], &newcan[0..i]) {
            return (i, can);
        }
    }
    return (largest, can);
}

fn candidate_overlaps_target(tar: &[u8], can: &[u8]) -> bool {
    if strand_is_empty(can) {
        println!("empty candidate strand");
        return false;
    } else if !strands_are_equal_len(tar, can) {
        println!("strands don't match in size: {:?} != {:?}", tar, can);
        return false;
    }
    if tar == can {
        return true;
    }
    return false;
}

fn strands_are_equal_len(tar: &[u8], can: &[u8]) -> bool {
    return tar.len() == can.len();
}

fn strand_is_empty(strand: &[u8]) -> bool {
    return strand.len() == 0
}

fn file_to_list(filename: &str) -> Vec<std::string::String> {
    let mut some_vec: Vec<std::string::String> = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for rez in lines {
            if let Ok(line) = rez {
                some_vec.push(line);
            }
        }
    }
    return some_vec;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
