fn main() {
    let x: i32 = 5;
    
    {
        let x: i32 = 42;
        println!("{:?}", x);
    }
    
    println!("{:?}", x);
}
