fn main() {
    let mut s = String::from("hello");

    println!("Before : {}", s); // hello
    change(&mut s);
    println!("After : {}", s); // hello, world
}

fn change(some_string: &mut String) { // some_string은 가변
    some_string.push_str(", world"); // 문자열 추가 가능
}