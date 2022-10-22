use std::io;
use std::io::Write; // The reason we also import the trait Write is that when I use a .flush(), it comes from the trait Write, and Rust being annoying as it usually is, requires you have the trait in the scope for you to stuff from it
use rand::Rng; // used to generate a random number
use std::cmp::Ordering; // used to compare

fn main() {
	let green_bold = "\x1b[1;92m";
	let reset = "\x1b[0m";
	let blue_italics = "\x1b[3;94m";
	let yellow_italics = "\x1b[3;93m";
	let red_italics = "\x1b[3;91m";

	println!("{}Guess the number {}", green_bold, reset);

	// the number of turns we want the user to get to guess the number
	let mut turns = 3;
	let low = 1;
	let high = 9;

	// generates a random number between 'low' to 'high'
	let rand_num = rand::thread_rng().gen_range(low..(high+1));

	println!("{}A random number between {} and {} has been selected", yellow_italics, low, high);

	// I love how Rust has their for loops xD
	while turns > 0 {
		turns -= 1;
		print!("{}Please enter your guess: {}", blue_italics, yellow_italics);
		io::stdout().flush().unwrap();

		// using 'mut' to denote that the variable is not a constant
		let mut guess = String::new(); // on testing it seems that .read_line only seemingly appending the input (not quite but somewhat? idk what I'm saying)

		io::stdin()
			// we use &mut to say that the given parameter is gonna take in an input which the function can take without getting 'ownership' over it
			.read_line(&mut guess)
			// Since .read_line returns a Result thingy, we need to deal with the error case
			.expect("Failed to read line to get the input");

		print!("{}", reset);
		io::stdout().flush().unwrap();

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("{}> Invalid input received, please enter a numeric guess {}", yellow_italics, reset);
				turns += 1;
				continue
			},
		};

		match guess.cmp(&rand_num) {
			Ordering::Less => println!("{}> You underestimate my number picking capabilities 😈 {}", red_italics, reset),
			Ordering::Greater => println!("{}> You overestimate my number picking capabilities 😒 {}", red_italics, reset),
			Ordering::Equal => {
				println!("{}> You got it absolutely correct! 😘🎉 {}", green_bold, reset);
				return
			}
		}

		if turns > 0 {
			// user still has more turns to play
			println!("{}You have {} turns left{}", yellow_italics, turns, reset);
		}
		else {
			println!("{}> Looks like this time, I emerge to be the winner 🤠😈{}", red_italics, reset);
		}
	}
}
