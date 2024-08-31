fn main() {
    // Variable are immutable by default so we need to use the mut keyword to make them mutable
    // Variable Shadowing
    let x = 5;
    println!("The value of first x is: {}", x);
    let x = "six";
    println!("The value of second x is: {}", x);

    // Mutability
    let mut y = 5;
    println!("The initial value of y is: {}", y);
    y = 10;
    println!("The new value of y is: {}", y);

    // Constants
    const value: u32 = 100_000;
}
