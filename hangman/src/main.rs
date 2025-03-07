use std::io;

fn guess_letter(secret_word: &str) -> bool {
    let mut guessed_letter = String::new();

    io::stdin()
        .read_line(&mut guessed_letter)
        .expect("Failed to read input");

    secret_word.contains(guessed_letter.trim())
}

fn main() {
    // ------- params
    let mut allowed_attemps = 5;
    // ------- params

    let mut secret_word = String::new();
    println!("Enter the secret word: ");

    io::stdin()
        .read_line(&mut secret_word)
        .expect("Failed to read input");

    while allowed_attemps > 0 {
        guess_letter(&secret_word);
        allowed_attemps -= 1;
    }
}
