
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
        let num1 = num1_string.parse::<f64>().unwrap();

    println!("You selected {}", num1_string);

    println!("Please type the second number and hit enter.");

        num2_string.clear();
        io::stdin().read_line(&mut num2_string).unwrap();
        num2_string = num2_string.trim().to_string();
        let num2: f64 = num2_string.parse().unwrap();

    println!("You selected {}", num2_string);

    println!("Please type a math symbol (* , / , + , -) and hit enter.");

        sym.clear();
        io::stdin().read_line(&mut sym).unwrap(); 
        sym = sym.trim().to_string();

    println!("You selected {}", sym);

    solve(num1,sym,num2);



}

    fn solve (num1: f64,sym:String,num2:f64){
        let answer:f64;   
        
        if sym == "*" {
            answer = num1*num2;
            println!("The answer is {}", answer);   
        }
        else if sym == "/" {
            answer = num1/num2;
            println!("The answer is {}", answer);
        }
        else if sym == "-"  {
            answer = num1-num2;
            println!("The answer is {}", answer);
            
        }
        else if sym == "+" {
            answer = num1+num2;
            println!("The answer is {}", answer);
        }
        else {
            println!("Sorry. This is not a viable problem. Try again.")
        };

     rerun()
    }

    fn rerun () {

        let mut rerun = String::new();
        println!("Would you like to solve another problem? (y/n)");
        rerun.clear();
        io::stdin().read_line(&mut rerun).unwrap();
        rerun = rerun.trim().to_string();

        if rerun == "y"{
            println! ("ok.");
            main ();
        }
        else {
            println! ("Bye.");
            std::process::exit(0)
        }
        
    }



  
    /* 
    while input_string != "x" { // This is the part that doesn't work right
        input_string.clear(); // First clear the String. Otherwise it will keep adding to it
        io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
        println!("You wrote {}", input_string);
    }
    */

