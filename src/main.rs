use std::io;


fn main() {
    /*let mut w = Word::new("asdfg");

    let guess = get_valid_guess(&w);

    w.add_to_revealed(guess);

    println!("{} {:?}", w.word, w.tried_chars);*/

    let mut game = Game::new();

    loop {
        game.play_round();
        if game.ongoing == false { break; }
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
            lives_remaining: 10,
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

    fn play_round(&self) {
        let guess = self.get_valid_guess();

        if self.word.contains(guess) {
            // Add to tried and reveal
        } else {
            // Add to tried and increment drawing
        }

        // End game if no lives left
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

