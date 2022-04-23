fn main() {
    // let reference_to_nothing = dangle();
    let reference_s = no_dangle();

    println!("{}", reference_s);
}

fn dangle() -> &String { // String reference를 반환
    let s = String::from("hello");

    &s // String s의 reference를 반환
} // s의 소유권은 여전히 해당 스코프 안에 있으므로, 스코프를 벗어나면 s의 메모리 해제
  // s의 reference 접근 불가

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}