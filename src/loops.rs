pub fn run() -> i32 {
    let mut count = 0;
    let mut max = 20;
    //infinite loop
    println!("infinite loop");
    loop {
        count += 1;
        println!("Number: {}", count);
        if count == max {
            break;
        }
    }
    //While loop
    count = 0;
    max = 100;
    println!("while loop");
    while count <= max {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count);
        }
        count += 1;
    }
    //For Range
    println!("for range");
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x);
        }
    }
    return 0;
}
