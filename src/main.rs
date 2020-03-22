use std::io;


fn main() {
    let mut w = Word::new("asdfg");
    let guess: char = get_player_guess();
    w.reveal(guess);
    println!("{} {:?}", w.word, w.revealed_chars);
}

fn get_player_guess() -> char {
    loop {
        println!("Please enter a single alphabetic character: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if char::is_alphabetic(guess) {
            return guess
        }
    }
}

struct Word {
    word: String,
    revealed_chars: Vec<char>,
}

impl Word {
    fn new(word: &str) -> Self {
        Word {
            word: word.to_string(),
            revealed_chars: Vec::<char>::new(),
        }
    }

    fn reveal(&mut self, c: char) {
        self.revealed_chars.push(c);
    }
}
