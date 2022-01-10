use std::process::Command;
use text_io::read;
use Err;

fn main() {
    println!("Hi! This program will calculate your caloric balance for the day!\nBefore we can start, I need some information about you. Be honest! :)\n");

    // gather info
    let mut gender: String = "x".to_string();
    let mut age = -1.0;
    let mut height = -1.0;
    let mut weight = -1.0;

    while gender != "f" && gender != "m" {
        println!("\nWhat is your gender (f or m)?: ");
        gender = read!("{}\n");
    }
    while age <= 0.0 {
        println!("\nWhat is your age as a whole number?: ");
        let temp: String = read!("{}\n");
        age = match temp.parse::<f64>() {
            Ok(v) => v,
            Err(_) => -1.0
        };
    }
    while height <= 0.0 {
        println!("\nWhat is your height in inches?: ");
        let temp: String = read!("{}\n");
        height = match temp.parse::<f64>() {
            Ok(v) => v,
            Err(_) => -1.0
        };
    }
    while weight <= 0.0 {
        println!("\nWhat is your weight in pounds? ");
        let temp: String = read!("{}\n");
        weight = match temp.parse::<f64>() {
            Ok(v) => v,
            Err(_) => -1.0
        };
    }

    // Men:   BMR = 66 + (12.7 * height in inches) + (6.23 * weight in pounds) - (6.8 * age in years)
    // Women: BMR = 655 + (4.7 * height in inches) + (4.35 * weight in pounds) - (4.7 * age in years)
    let mut daily_cals = 0.0;
    if gender == "m" {
        daily_cals += 66.0 + (12.7 * height) + (6.23 * weight) - (6.8 * age);
    } else {
        daily_cals += 655.0 + (4.7 * height) + (4.35 * weight) - (4.7 * age);
    }

    let mut caloric_balance = 0.0 - daily_cals;

    println!("\nThanks! Now, throughout the day, tell me each time you eat or move.\nYour caloric balance is starting at {} (you need to eat more!)\n", caloric_balance);
    
    let mut quitting = false;

    // add workouts and meals
    while !quitting {
        println!("\nWhat would you like to do?\n\n[f] Record Food Consumption\n[a] Record Physical Activity\n[q] Quit\n\nEnter an option: ");
        let decision: String = read!("{}\n");
        
        if decision == "q" {
            println!("Leaving? You should do this again tomorrow. Stay healthy!");
            quitting = true;
        } else if decision == "f"{
            let mut calories = -1.0;
            while calories <= 0.0 {
                println!("\nOkay, how many calories did you eat?: ");
                let temp: String = read!("{}\n");
                calories = match temp.parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => -1.0
                };
            }
            caloric_balance += calories;
            println!("Awesome, your caloric balance is {}.", caloric_balance)
        } else if decision == "a" {
            let mut activity = String::new();

            while activity.len() == 0 {
                println!("\nOkay, what activity did you do?\n[r] Run\n[w] Walk\n[s] Swim\n[b] Ride Bike\n[l] Lift");
                activity = read!("{}\n");
                if !"rwsbl".contains(&activity) {
                    println!("\nPlease select one of the activities listed in lower case.");
                    activity = "".to_string();
                }
            }
            let mut minutes = -1.0;
            while minutes <= 0.0 {
                println!("\nHow long were you active in minutes?: ");
                let temp: String = read!("{}\n");
                minutes = match temp.parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => -1.0
                };
            }
            let mut mult = 0.0;
            let arr = "r".to_string();
            let dblu = "w".to_string();
            let ess = "s".to_string();
            let bee = "b".to_string();
            let ell = "l".to_string();
            match &activity {
                arr => mult = 11.4,
                dblu => mult = 3.73,
                ess => mult = 9.07,
                bee => mult = 9.93,
                ell => mult = 6.67
            }

            caloric_balance -= mult * minutes;

            println!("\nCool, your caloric balance is {}.\n", caloric_balance)
        } else {
            println!("Please enter a valid response.\n");
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
    }
}
