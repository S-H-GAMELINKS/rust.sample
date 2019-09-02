fn main() {
 
    let i = 42;
    
    {
        let i = 4;
        println!("{}", i);
    }
    
    println!("{}", i);
}
