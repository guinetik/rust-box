/*
    Primitive Types:
        Integers: u8, i8, u16, u32, i32, u64, i64, u128, i128
        floats: f32, f64
        boolean: true, false
        character: single string
        Tuples
        Arrays: fixed length
*/
pub fn run() {
    // Rust is statically typed, so the compiler must know the types of all variables, however the compiler can also infer what type to use based on value.
    // default type for numbers is i32;
    let x = 1;
    //default vaule for floats if f64
    let y = 5.5;
    //adding explicit type
    let z:i64 = 12345678909;
    // find max value for a given type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    // boolean
    let is_active:bool = true;
    // boolean as a result of expression
    let is_greater: bool = 2 > 1;
    //char - uses single quote
    let a1 = 'a';
    let emoji = '\u{26A1}';
    println!("Rust is {} fast", emoji);
    println!("{:?}", (x, y, z, is_active, is_greater, a1));
}