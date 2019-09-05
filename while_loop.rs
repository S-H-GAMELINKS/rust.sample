fn main() {
    let mut i = 0;
    let mut done = false;
    
    while !done {
        println!("{}", i);
        i += 1;
        
        if i != 0 && i % 5 == 0 {
            done = true;   
        }
    }
}
