enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon
}

impl Shape {
    fn num(self) -> i32 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 3,
            Shape::Octagon => 8


            
        }
    }
    
}
fn main() {
    println!("Hello, world!");
}
