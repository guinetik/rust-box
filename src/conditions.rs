pub fn run() -> i32 {
    let age:u8 = 18;
    let mut check_id: bool = false;
    let knows_persons_age = false;
    println!("Checking age for new customer...");
    check_age(age, check_id, knows_persons_age);
    check_id = true;
    println!("Already checked this customer...");
    check_age(age, check_id, knows_persons_age);
    println!("{}", check_id);
    println!("Checking age for known customer...");
    check_id = false;
    check_age(0, check_id, true);
    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);
    return 0;
}

fn check_age(age:u8, check_id:bool, knows_persons_age:bool) {
    if age  >= 21 || knows_persons_age {
        println!("What's your drink?")
    } else if age <21 && check_id {
        println!("GTFO!")
    } else {
        println!("Let's see some ID");
    }
}