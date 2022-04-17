fn main() {
    loop {
        println!("loop again!");
        break
    }
    
    let mut number = 3;

    while number != 0 {
        println!("number is : {}", number);
        number = number - 1;
    }
    println!("LIFT OFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is : {}", element);
    }

    for element in (1..4).rev() {
        println!("{}!", element);
    }
    println!("LIFTOFF!!!");
}
