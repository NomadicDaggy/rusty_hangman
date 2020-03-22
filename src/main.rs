fn main() {
    let mut w = Word::new("asdfg");
    w.reveal('a');
    println!("{} {:?}", w.word, w.revealed_chars);
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
