
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
    let mut num1:i128;
    let mut sym = String::new();
    let mut num2:i128;
    let mut answer:i128;

        if sym == "*" {
            let answer = num1*num2;   
        }
        else if sym == "/" {
            let answer = num1/num2;
        }
        else if sym == "-"  {
            let answer = num1-num2;
            
        }
        else if sym == "+" {
            let answer = num1+num2;
        }
        else {
            println!("Sorry. This is not a viable problem. Try again.")
        };
    println!("The answer is {}", answer);

    }


  
    /* 
    while input_string != "x" { // This is the part that doesn't work right
        input_string.clear(); // First clear the String. Otherwise it will keep adding to it
        io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
        println!("You wrote {}", input_string);
    }
    */

