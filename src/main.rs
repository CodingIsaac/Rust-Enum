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
    let triangle = Shape::Triangle;
    let square = Shape::Square;
    let pentagon = Shape::Pentagon;
    let octagon = Shape::Octagon;

    println!("{:?}", triangle.num());
    println!("{:?}", square.num());
    println!("{:?}", pentagon.num());
    println!("{:?}", octagon.num());
   
}
