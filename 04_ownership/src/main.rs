fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    let x = 5;
    let y = x;

    let x = 7;

    println!("y: {}, x: {}", y, x);

    let t1 = gives_ownership();
    let t2 = String::from("Hello");
    let t3 = takes_and_gives_back(t2);

    // Mutability
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
    println!("First word: {}", first_word(&String::from("привет, мир")));
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
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
