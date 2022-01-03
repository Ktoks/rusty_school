use std::process::Command;
use text_io::read;
use Err;

#[derive(Debug, Clone)]
struct Trip {
    date_left: String,
    traveled_miles: f64,
    gas_pumped: f64
}

impl Trip {
    fn make_trip(date: String, traveled: f64, gas: f64) -> Trip {
        Trip{date_left: date, traveled_miles: traveled, gas_pumped: gas}
    }

    fn show_trip(self) -> String {
        let trip_info = ["On ", &self.date_left, " you traveled ", &self.traveled_miles.to_string(), " miles using ", &self.gas_pumped.to_string(), " gallons."].concat();
        return trip_info;
    }

    fn miles_per_gallon(self) -> f64 {
        return self.traveled_miles / self.gas_pumped;
    }
}

fn main() {
    println!("Welcome to Gas Mileage Calculator!\n");

    let mut quitting = false;

    let mut trips: Vec<Trip> = Vec::new();

    while !quitting {
        println!("What would you like to do?\n[r] Record Gas Consumption\n[l] List Mileage History\n[c] Calculate Gas Mileage\n[q] Quit\nEnter an option: ");
        let decision: String = read!("{}\n");

        if decision == "q" {
            println!("\nThanks for using Gas Mileage Caluculator!");
            quitting = true;
        } else if decision == "l" {
            let temp = trips.clone();
            if temp.len() > 0 as usize {
                for trip in temp {
                    println!("{}", &trip.show_trip());
                }
            } else {
                println!("You first need to record your gas consumption!");
            }
        } else if decision == "c" {
            if trips.len() < 1 {
                println!("There are no trips to calculate!");
            } else {
                let mut total = 0.0;
                let temp = trips.clone();
                for t in temp {
                    total += t.miles_per_gallon();
                }
                total = total / trips.len() as f64;
                println!("\nAverage gas mileage: {}.\n", total)
            }
        } else if decision == "r" {
            let mut the_date = String::new();
            while the_date.len() == 0 {
                println!("What is the date?: ");
                the_date = read!("{}\n");
            }
            let mut miles = 0.0;
            while miles <= 0.0 {
                println!("How many miles did you drive since last filling up? ");
                let some: String = read!("{}\n");
                miles = match some.parse() {
                    Ok(v) => v,
                    Err(_) => 0.0
                };
                if miles <= 0.0 {
                    println!("Please enter a positive number in the format 1 or 1.5.");
                    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
                    let _result = child.wait().unwrap();
                }
            }
            let mut gallons = -1.0;
            while gallons < 0.0 {
                println!("How many gallons did you add to your tank? ");
                let some: String = read!("{}\n");
                gallons = match some.parse() {
                    Ok(v) => v,
                    Err(_) => -1.0
                };
                if gallons < 0.0 {
                    println!("Please enter a positive number in the format 1 or 1.5");
                    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
                    let _result = child.wait().unwrap();
                }
            }
            let trippy = Trip::make_trip(the_date, miles, gallons);
            trips.push(trippy);
            println!("Saved!");
        } else {
            println!("{}'s not a valid input!", decision);
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
    }
}
