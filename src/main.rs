
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

    let mut sym = String::new();
    let mut num1_string = String::new();
    let mut num2_string = String::new();

    println!("Please type the first number and hit enter.");

        num1_string.clear();
        io::stdin().read_line(&mut num1_string).unwrap();
        num1_string = num1_string.trim().to_string();
        let num1 = num1_string.parse::<i128>().unwrap();

    println!("Please type the second number and hit enter.");

        num2_string.clear();
        io::stdin().read_line(&mut num2_string).unwrap();
        num2_string = num2_string.trim().to_string();
        let num2: i128 = num2_string.parse().unwrap();

    println!("Please type a math symbol (* , / , + , -) and hit enter.");

        sym.clear();
        io::stdin().read_line(&mut sym).unwrap(); 
        sym = sym.trim().to_string();

    solve(num1,sym,num2);


}

    fn solve (num1: i128,sym:String,num2:i128){
        let mut answer:i128=0;   
        
        if sym == "*" {
            answer = num1*num2;   
        }
        else if sym == "/" {
            answer = num1/num2;
        }
        else if sym == "-"  {
            answer = num1-num2;
            
        }
        else if sym == "+" {
            answer = num1+num2;
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

