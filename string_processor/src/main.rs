use std::io;

// AI 에게 질의한 클래스 코드에서 모듈화 진행한 뒤 visible 하게 변경
mod string_module {
    pub struct StringProcessor {
        input: String,
    }

    impl StringProcessor {
        pub fn new(input: String) -> StringProcessor {
            StringProcessor { input }
        }
        pub fn to_uppercase(&self) -> String {
            self.input.to_uppercase()
        }
        pub fn to_lowercase(&self) -> String {
            self.input.to_lowercase()
        }
        pub fn len(&self) -> usize {
            self.input.len()
        }
        pub fn find(&self, c: char) -> Option<usize> {
            self.input.find(c)
        }
        pub fn compare(&self, other: &str) -> bool {
            self.input == other
        }
    }
}

fn main() {
    // AI 작성한 코드에서 모듈화를 진행했다면, use 키워드를 추가해야 합니다.
    use string_module::StringProcessor;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    let processor = StringProcessor::new(input);
    // println!("Original: {}", processor.input); // PRIVATE 필드로 오류!
    println!("Uppercase: {}", processor.to_uppercase());
    println!("Lowercase: {}", processor.to_lowercase());
    println!("Length: {}", processor.len());
    println!("Find 'o': {:?}", processor.find('o'));
    println!("Compare 'Hello': {}", processor.compare("hello"));
}