
// This is similir to the arrow function of JavaScript or Lamda function of java
fn add(x: i32, y: i32) -> i32{
    x + y  // no semicolon for returing value
}

fn main(){
    let sum = add(3, 5);
    println!("Sum: {}", sum)
}