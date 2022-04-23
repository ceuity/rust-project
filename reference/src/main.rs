fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&s1)
}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자
    s.len()
} // s는 스코프를 벗어나도 값에 대한 소유권이 없기 때문에 아무런 일이 발생하지 않음

fn change(some_string: &String) {
    some_string.push_str(", world");
}