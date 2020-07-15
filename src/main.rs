use std::io::{stdin, stdout, Write};

fn ask_user_for_replay() -> bool {
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
        false
    } else {
        true
    }
}

fn main() {
    loop {
        let mut a = String::new();
        let mut b = String::new();

        // Clear screen
        print!("{}[2J", 27 as char);

        // Introduce game
        println!("Greetings, O Curious One! Care to spare a few numeric inputs for a humble bot? I promise, it will be worth your while.");

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

        // Respond depending on validity of inputs
        let a = a.parse::<i32>();
        let b = b.parse::<i32>();
        if a.is_err() || b.is_err() {
            println!("O Sly One, it looks like you have provided me with improper inputs.");
            println!("Addition under these conditions would be futile.");
        } else {
            let a = a.unwrap();
            let b = b.unwrap();
            println!("Thank you for providing me such immaculate numbers, O Great One!\nBehold, now, the power of summation:");
            println!("\n\t{} + {} = {}", a, b, a + b);
            stdout().flush().unwrap();
        }

        // Ask user if they want to replay, otherwise break
        let wants_replay = ask_user_for_replay();
        if !wants_replay {
            break;
        }
    }
}
