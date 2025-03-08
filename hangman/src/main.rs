use std::io;

fn guess_letter() -> char {
    let mut input = String::new();
    println!("What letter would you like to guess ? : ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().chars().next().unwrap_or('_')
}

fn main() {
    // ------- params
    let mut allowed_guesses = 5;
    // ------- params

    let mut secret_word = String::new();
    println!("Enter the secret word: ");
    io::stdin()
        .read_line(&mut secret_word)
        .expect("Failed to read input");

    let secret_word = secret_word.trim().to_string();
    let mut discovered_word = "_".repeat(secret_word.len());

    while allowed_guesses > 0 {
        let guessed_letter = guess_letter();
        discovered_word = secret_word
            .chars()
            .zip(discovered_word.chars())
            .map(|(w, m)| {
                if m == '_' && w == guessed_letter {
                    w
                } else {
                    m
                }
            })
            .collect();
        println!(
            "Current word status : {}, remaining guesses : {}\n",
            discovered_word, allowed_guesses
        );
        allowed_guesses -= 1;
        if discovered_word == secret_word {
            println!("Congratulations ! You found the word : {}", discovered_word);
            break;
        }
    }
    if allowed_guesses == 0 {
        println!("You lost... You got this far : {}", discovered_word);
    }
}
