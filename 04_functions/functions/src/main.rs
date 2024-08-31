fn main() {
    my_function();
    let sum = my_output(10, 20);
    println!("Sum: {}", sum);
}

fn my_function() {
    println!("This is my function.");
}

fn my_output(x: i32, y: u32) -> i32 {
    println!("x: {}, y: {}", x, y);
    let sum = x + y as i32;
    return sum;
}
