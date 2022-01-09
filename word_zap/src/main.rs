use text_io::read;
use Err;
use rand::{Rng};

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const LETTER_COLLECTION_LEN: usize = 6;


fn main() {
    println!("Welcome! Time to play! Try to use all of your letters.");
    println!("The first player that uses all of their letters wins!");

    println!("How many players will be playing? ");

    let str_num_players: String = read!("{}\n");
    
    let num_players = match str_num_players.parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0
    };
    if num_players == 0 {
        println!("If you want to play sometime, enter real numbers.");
        1;
    }

    let mut name_list: Vec<String> = std::vec::Vec::new();
    let mut let_list: Vec<String> = std::vec::Vec::new();
    let mut winners: Vec<String> = std::vec::Vec::new();

    for i in 1..num_players+1 {
        println!("Enter a name for player #{}", i);
        let temp: String = read!("{}\n");
        name_list.push(temp);

        let mut rng = rand::thread_rng();
        let let_collection: String = (0..LETTER_COLLECTION_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
        let_list.push(let_collection);
    }

    println!("Great, now we can play!");

    let mut quit = false;
    while !quit {
        for i in 0..name_list.len() {
            // loop until broken
            loop {
                println!("\n\n{} it's your turn!\n\nYour letters are ({})", name_list[i], let_list[i]);
                let guess: String = read!("{}\n");
                let mut temp_let_list = let_list[i].clone();
                let mut all_good = true;
                // if there's no guess, add a letter
                if guess.len() == 0 {
                    let mut rng = rand::thread_rng();
                    let loc = rng.gen_range(0..CHARSET.len());
                    let letter = CHARSET[loc] as char;
                    println!("Skipped turn, you draw {}.", letter);
                    let_list[i].push(letter);
                    break;
                }
                // for each guessed letter
                for j in 0..guess.len() {
                    let mut matched = -1;
                    // check if each guess letter is in the let list (decreasing with matches)
                    for k in 0..temp_let_list.len() {
                        // if one matches
                        if guess.chars().nth(j).unwrap() == temp_let_list.chars().nth(k).unwrap() {
                            matched = k as i32;
                            break;
                        }
                    }
                    if matched > -1 {
                        temp_let_list.remove(matched as usize);
                    } else {
                        // this turn needs to continue
                        all_good = false;
                        println!("Check your list and try again!\n");
                    }
                }
                if all_good {
                    let_list[i] = temp_let_list;
                    // good turn, turn over
                    break;
                }
            }
            if let_list[i].len() == 0 {
                let winner: String = name_list[i].clone();
                winners.push(winner);
                quit = true;
            }
        }
    }
    // end of game
    println!("\nGame over!");
    if winners.len() > 1 {
        println!("\nTie game! The winners are:");
        for winner in winners{
            println!("   {}", winner);
        }
    } else {
        println!("\nThe winner is {}!\n", winners[0]);
    }
    println!("Thanks for playing!");
}
