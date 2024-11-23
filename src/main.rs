use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_num: u32 = rand::thread_rng().gen_range(1..=10);
    println!("Guessing Game in RUST...!\n");
    let mut cnt: i32 = 1;

    loop {
        if cnt > 3 {
            println!("You failed!");
            break;
        }
        println!("Trial {cnt}");
        let mut guess: String = String::new();
        println!("\nPlease input your number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a number bozo!");
                continue;
            },
        };

        match guess.cmp(&secret_num) {
            Ordering::Greater => {
                println!("The number is smaller than {guess}");
                cnt += 1;
            },
            Ordering::Less => {
                println!("The number is greater than {guess}");
                cnt += 1;
            },
            Ordering::Equal => {
                println!("Yay!!!\nYou guessed the number correctly!");
                break;
            },
        }
    }
}
