use text_io::read;

fn main() {
    let my_name= String::from("kacy");
    println!("Hello there! If you can guess my name in 3 guesses, you win a hearty slap on the back!");
    println!("Give it a go! ");
    let mut i: u8 = 0;
    let mut win: bool = false;
    while i < 3 {
        let  userin: String = read!("{}\n");
        if userin.trim().to_lowercase() == my_name {
            win = true;
            break;
        } else {
            println!("Ha! Incorrect!");
        }
        println!("You guessed {}...", userin);

        i += 1;
    }
    if win {
        println!("Congradulations! You guessed my name!");
    } else {
        println!("You lose!");
    }
}
