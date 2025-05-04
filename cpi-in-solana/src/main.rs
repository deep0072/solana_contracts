fn main(){
    let mut x = 90;
    let mut y = &mut x;
    *y+=90;
    println!("x ,y{x}");


}