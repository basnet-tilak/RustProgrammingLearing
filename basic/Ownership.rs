
fn main(){
    let s1 = String::from("Hello"); // s1 owns the string
    // let s2 = s1; // s2 now owns the string, s1 is invalidated (move)
    // println!("{}", s1); // Error: s1 s no longer valid

    let len = calculate_length(&s1); // pass reference to the function
    println!("Thie length of {} is {}", s1, len);
}

fn  calculate_length(s: &String) -> usize {
    s.len()
}