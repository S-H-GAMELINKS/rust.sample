fn sum(x : i32, y : i32) -> i32 {
    x + y
}

fn main() {
 
    let (i, j) = (21, 21);
    
    let f : fn(i32, i32) -> i32 = sum;
    
    println!("{}", f(i, j));
}
