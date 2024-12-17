fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Cloning the value to avoid issues with the reference.
    vec.push(3); 
    println!("The value of x is: {}", x);
}

//Alternative solution using immutable reference and preventing further modification
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = &vec[0];
    println!("The value of x is: {}", x);
}
