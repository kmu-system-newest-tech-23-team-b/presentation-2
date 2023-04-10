struct Calculator {
    x: i32,
    y: i32,
    op: String,
}

impl Calculator {
    fn new(x: i32, y: i32, op: String) -> Calculator {
        Calculator { x, y, op }
    }
    fn add(&self) -> i32 {
        self.x + self.y
    }
    fn sub(&self) -> i32 {
        self.x - self.y
    }
    fn mul(&self) -> i32 {
        self.x * self.y
    }
    fn div(&self) -> f32 {
        if self.x == 0 || self.y == 0 {
            panic!("Divide by zero");
        } else {
            self.x as f32 / self.y as f32
        }
    }
    pub fn calc(&self) -> i32 {
        match self.op.as_ref() {
            "+" => self.add(),
            "-" => self.sub(),
            "*" => self.mul(),
            "/" => self.div() as i32,
            _ => panic!("Invalid operator!"),
        }
    }
}

fn main() {
    let obj = Calculator::new(3, 1, String::from("/"));
    println!("{}", obj.calc());
}