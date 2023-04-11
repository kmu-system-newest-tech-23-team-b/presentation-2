use std::io;

struct StringProcessor {
    input: String,
}

impl StringProcessor {
    fn new(input: String) -> StringProcessor {
        StringProcessor { input }
    }
    fn to_uppercase(&self) -> String {
        self.input.to_uppercase()
    }
    fn to_lowercase(&self) -> String {
        self.input.to_lowercase()
    }
    fn len(&self) -> usize {
        self.input.len()
    }
    fn find(&self, c: char) -> Option<usize> {
        self.input.find(c)
    }
    fn compare(&self, other: &str) -> bool {
        self.input == other
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    let processor = StringProcessor::new(input);
    println!("Original: {}", processor.input);
    println!("Uppercase: {}", processor.to_uppercase());
    println!("Lowercase: {}", processor.to_lowercase());
    println!("Length: {}", processor.len());
    println!("Find 'o': {:?}", processor.find('o'));
    println!("Compare 'Hello': {}", processor.compare("hello"));
}