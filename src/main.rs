
//fn main() {
//    println!("Hello, world!");
//    let a: i128 = 850432000;
//    let b: i128 = 999000000; 
//
//  let product: i128 = a*b;
// 
//    print!("Product = {}", product);
    
//}


use std::io;


fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
loop {
        if input_string == "stop" {
            println!("See you later!");
        }
        else {
        input_string.clear();

        io::stdin().read_line(&mut input_string).unwrap(); 

        println!("You wrote {}", input_string);
        }
    }
}
  
    /* 
    while input_string != "x" { // This is the part that doesn't work right
        input_string.clear(); // First clear the String. Otherwise it will keep adding to it
        io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
        println!("You wrote {}", input_string);
    }
    */

