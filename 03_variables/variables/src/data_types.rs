fn main() {
    // Scalar datatypes - Integers, Floats, Booleans, Characters

    // Integers - signed i8 to i128 and unsigned u8 to u128
    let a: i32 = 92_888;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e: u8 = b'A';

    // Floats - f32, f64
    let f = 2.0;
    let g: f32 = 3.0;

    // Booleans - true and false
    let h = true;
    let i = false;

    // Characters - Unicode Scalar Values
    let j = 'z';
    let k = '😻';

    // Compound datatypes - Tuples, Arrays

    // Tuples - Fixed length
    let l: (i32, f64, &str) = (500, 6.4, "Hello");
    let (m, n, o) = l;
    let x = l.0;

    // Arrays - Fixed length
    let p: [i32; 5] = [1, 2, 3, 4, 5];
}
