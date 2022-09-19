fn main() {
    
    println!("Hello, world!");
    let hash = String::from("foo");
    
    println!("{}", format!("0x{}", hex::encode(hash)));
}
