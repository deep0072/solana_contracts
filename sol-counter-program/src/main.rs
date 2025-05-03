fn main(){
    println!("Hello, world!");
    let mut x = String::from("hi there");
    let y = &mut x;
    y.push_str("string");
    println!("{y}")
}