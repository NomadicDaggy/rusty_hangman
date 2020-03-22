use std::io;


fn main() {
    let mut w = Word::new("asdfg");

    let guess = get_valid_guess(&w);

    w.add_to_revealed(guess);

    println!("{} {:?}", w.word, w.revealed_chars);
}

fn get_valid_guess(w: &Word) -> char {
    loop {
        let guess: char = cli_read_letter();
        if !w.revealed_chars.contains(&guess) {
            return guess
        }

        println!("You already guessed \"{}\"", guess);
    }
}

fn cli_read_letter() -> char {
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

    fn add_to_revealed(&mut self, c: char) {
        // At this point its guaranteed that char is fine
        self.revealed_chars.push(c);
    }
}
