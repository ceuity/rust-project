fn main() {
    let mut s = String::from("hello, world");
    let word = first_word(&s);
    
    println!("word is {}", word);
    
    s.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for &i in slice.iter() {
        println!("i = {}", i);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}