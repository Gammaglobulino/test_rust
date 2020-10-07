struct human{
    name:String,
}



fn main() {
    let v=[1,2,3,4];
    for i in &v{
        print!("{}",i);
    }
    println!("Hello, world!");
}