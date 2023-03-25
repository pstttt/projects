use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Parser)]
struct Arguments {
    #[arg(short, long, default_value_t = 16)]
    length: u16,
    #[arg(short, long, default_value = "abcdefghijklmnopqrstuvwxyz")]
    random_string: String,
}

fn get_random_character(string: &str) -> char {
    let mut rng = thread_rng();

    let characters = string.chars().collect::<Vec<char>>();
    let character_index: usize = rng.gen_range(0..characters.len());
    *characters.get(character_index).unwrap()
}

fn main() {
    let arguments = Arguments::parse();

    for _ in 1..=arguments.length {
        print!("{}", get_random_character(&arguments.random_string));
    }; println!();
}
