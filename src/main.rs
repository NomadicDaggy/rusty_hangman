use std::io;


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

impl Word {
    fn new(word: &str) -> Self {
        Word {
            word: word.to_string(),
            tried_chars: Vec::<char>::new(),
        }
    }

    fn add_to_revealed(&mut self, c: char) {
        // At this point its guaranteed that char is fine
        self.tried_chars.push(c);
    }
}

struct Game {
    word: Word,
    lives_remaining: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            word: Word::new("testerino"),
            lives_remaining: 3,
        }
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
        self.word.tried_chars.push(guess);

        if self.word.word.contains(guess) {
        } else {
            // Add to tried and increment drawing
            self.lives_remaining -= 1;
        }

        // End game if no lives left
        if self.lives_remaining == 0 {
            false
        } else {
            true
        }
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

