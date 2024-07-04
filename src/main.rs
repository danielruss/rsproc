use std::str;
fn main() {
    println!("Hello, world! ");

    let s = match str::from_utf8(&[0x25,0x00,0x25,0x02]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}
