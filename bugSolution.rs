fn main() {
    let mut x = 5;
    { // Create a new scope to limit lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifying x through y
        println!("x: {}", x); // x is 6
    }
    let z = &x; // z is an immutable reference to x
    println!("x: {}", x); 
}