
struct Rectangle{
    width: u32,
    height: u32,
}
// Implementatio block for the Rectangle struct
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width + self.height
    }
    // Method to calculate the perimeter of the rectangle
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main(){
    // Create an instance of Rectangle
    let rect = Rectangle{ width: 30, height: 50 };
    
    // Call the methods of the Rectangle struct
    println!("Area of the rectangle: {}", rect.area());
    println!("Perimeter of the rectangle: {}", rect.perimeter());
}