use std::io;
use std::fmt;


fn main() {
    let mut game = Game::new();

    loop {
        // This could read better
        if !game.play_round() { break };
    }
}


struct Word {
    word: String,
    tried_chars: Vec<char>,
}

impl fmt::Display for Word {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut encoded_word = self.word.clone();

        for c in self.word.chars() {
            if !self.tried_chars.contains(&c) {
                encoded_word = encoded_word[..].replace(c, "_");
            }
        }

        // Write into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", encoded_word)
    }
}

impl Word {
    fn new(word: &str) -> Self {
        Word {
            word: word.to_string(),
            tried_chars: Vec::<char>::new(),
        }
    }

    fn add_to_tried(&mut self, c: char) {
        // At this point its guaranteed that char is fine
        self.tried_chars.push(c);
    }

    fn guessed_all(&self) -> bool {
        // Does tried_chars contain all the unique chars of word
        let mut unique_chars: Vec<_> = self.word.clone().chars().collect();
        unique_chars.dedup();

        for c in unique_chars.iter() {
            if !self.tried_chars.contains(c) { return false }
        }

        true
    }
}

struct Game {
    word: Word,
    lives_remaining: usize,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            word: Word::new("testerino"),
            lives_remaining: 2,
        };

        game.word.add_to_tried(game.word.word.chars().next().unwrap());
        println!("{}", game.word);

        game
    }

    fn get_valid_guess(&self) -> char {
        loop {
            let guess: char = cli_read_letter();
            if !self.word.tried_chars.contains(&guess) {
                return guess
            }
    
            println!("You already guessed \"{}\"", guess);
        }
    }

    fn play_round(&mut self) -> bool{
        // Display word

        let guess = self.get_valid_guess();
        self.word.add_to_tried(guess);

        if self.word.word.contains(guess) {
        } else {
            // Add to tried and increment drawing
            self.lives_remaining -= 1;
        }

        println!("{}", self.word);

        if self.word.guessed_all() {
            println!("You won!");
            false
        } else if self.lives_remaining == 0 {
            println!("You lost!");
            false
        } else { true }
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

