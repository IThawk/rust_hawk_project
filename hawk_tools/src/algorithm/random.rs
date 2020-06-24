use rand::prelude::*;

const CHARS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn random_int(max: i32) -> i32 {
    let mut rng = thread_rng();
    let n: i32 = rng.gen_range(0, max);
    println!("{}", n);
    return n;
}

pub fn make_random_string(length: i32) -> String {
    let mut result = String::new();

    for i in 0..length {
        let c: &char = CHARS.get(random_int(36) as usize).unwrap();
        result = format!("{}{}", result, c);
    }
    println!("{}", result);
    result
}

#[test]
fn random_int_test() {
    let s = random_int(10);
    println!("{:?}", s);
}

#[test]
fn make_random_string_test() {
    let s = make_random_string(32);
    println!("{:?}", s);
}
