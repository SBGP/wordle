use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashSet;
use rand::{thread_rng, seq::SliceRandom};

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let dictionary = load_dictionary("dictionary.txt").expect("Failed to load dictionary");
    let dictionary_vec: Vec<_> = dictionary.iter().cloned().collect();

    let secret_word = dictionary_vec.choose(&mut thread_rng())
                        .expect("Dictionary is empty")
                        .to_string();

    let mut attempts = 6;
    let mut previous_guesses = Vec::new();

    println!("welcome to wordle!");
    println!("guess the 5-letter word. you have {} attempts.", attempts);

    while attempts > 0 {
        print!("enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().to_lowercase();

        if guess.len() != 5 || !dictionary.contains(&guess) {
            println!("{}", "please enter a valid 5-letter english word.".red());
            continue;
        }

        let feedback = guess
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if secret_word.chars().nth(i).unwrap() == c {
                    c.to_string().green()
                } else if secret_word.contains(c) {
                    c.to_string().yellow()
                } else {
                    c.to_string().red()
                }
            })
            .fold(String::new(), |mut acc, cs| {
                acc += &cs.to_string();
                acc
            });

        previous_guesses.push((guess.clone(), feedback));

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("welcome to wordle!");
        println!("guess the 5-letter word. you have {} attempts.", attempts);

        for (guess, feedback) in &previous_guesses {
            println!("{} - {}", guess, feedback);
        }

        if guess == secret_word {
            println!("{}", "congratulations, you've guessed the word!".green());
            return;
        }

        attempts -= 1;
        println!("attempts left: {}", attempts);
    }

    println!("{}", format!("the word was '{}'.", secret_word).red());
}

fn load_dictionary(path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let dictionary = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<HashSet<String>>();
    Ok(dictionary)
}