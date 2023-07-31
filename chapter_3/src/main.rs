fn main() {
    let mut test: i32 = 20;
    ownership_test(&mut test);
    println!("ownership_test {}", test);
}

fn ownership_test(data: &mut i32) {
    *data += 10;
}
