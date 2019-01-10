fn main() {
    let x: i32 = 42;
    let y: i32 = 38;
    
    println!("{:?}", sum(x, y));
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}
