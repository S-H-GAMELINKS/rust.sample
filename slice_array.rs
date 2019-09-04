fn main() {
    let a = [0; 10];
    let s = &a[..];
    let c = &a[1..4];
    
    println!("{}", a.len());
    println!("{}", s.len());
    println!("{}", c.len());
}
