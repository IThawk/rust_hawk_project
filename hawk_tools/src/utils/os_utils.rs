pub fn is_windows() -> bool {
    if cfg!(windows) {
        info!("this os is windows");
        true
    } else {
        info!("this os is unix");
        false
    }
}

#[test]
fn write_test() {
    let point = is_windows();
    println!("{:?}", point)
}
