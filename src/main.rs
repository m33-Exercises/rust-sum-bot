use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut a = String::new();
        let mut b = String::new();

        print!("{}[2J", 27 as char);

        // Get first input
        println!("");
        print!("Input A: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut a).expect("Failed to read input.");
        let a = a.trim_end();

        // Get second input
        print!("Input B: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut b).expect("Failed to read input.");
        let b = b.trim_end();

        // Do some error checking here

        // If valid input, give sum
        println!("Thank you for your numbers and your patience, O Great One!\nBehold:");
        println!("\n\t{} + {} = {}", a, b, "5");
        stdout().flush().unwrap();

        // Ask user if they want to replay, otherwise break
        let mut replay = String::new();
        println!("");
        print!("The math, it beguiles you. Do you wish to play again (y/n)? ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut replay)
            .expect("No replay input read.");

        let first = replay
            .chars()
            .next()
            .expect("Couldn't extract a character.");
        if first != 'y' {
            println!("I see. I will look forward to our next encounter then, O Great One. Until then.");
            break;
        }
    }
}
