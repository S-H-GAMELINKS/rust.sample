fn main() {
    let f: fn(i32) -> i32 = plus;
    
    println!("This number is Six : {:?}", f(5));
}

fn plus(i: i32) -> i32 {
    i + 1   
}
