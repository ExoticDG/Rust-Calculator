
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

    println!("Please type the first number and hit enter. (if you want to use pi type 3.14 in this)");

        num1_string.clear();
        io::stdin().read_line(&mut num1_string).unwrap();
        num1_string = num1_string.trim().to_string();
        let num1 = num1_string.parse::<f64>().unwrap();

    println!("You selected {}", num1_string);

    println!("Please type the second number and hit enter. (if you want to use pi type 3.14 in this)");

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
        let mut answer:f64;  
        let pi1:f64;
        let pi2:f64;

        if  num2 == 3.14 {
            pi2 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989;


            if sym == "*" {
                answer = num1*pi2;
                println!("The answer is {}", answer);   
            }
            else if sym == "/" {
                answer = num1/pi2;
                println!("The answer is {}", answer);
            }
            else if sym == "-"  {
                answer = num1-pi2;
                println!("The answer is {}", answer);
                
            }
            else if sym == "+" {
                answer = num1+pi2;
                println!("The answer is {}", answer);
            }
            else {
                println!("Sorry. This is not a viable problem. Try again.")
            };


        }
        else {

        }

        if  num1 == 3.14 {
            pi1 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989;
            println!("{}", pi1);

            if sym == "*" {
                answer = pi1*num2;
                println!("The answer is {}", answer);   
            }
            else if sym == "/" {
                answer = pi1/num2;
                println!("The answer is {}", answer);
            }
            else if sym == "-"  {
                answer = pi1-num2;
                println!("The answer is {}", answer);
                
            }
            else if sym == "+" {
                answer = pi1+num2;
                println!("The answer is {}", answer);
            }
            else {
                println!("Sorry. This is not a viable problem. Try again.")
            };
        }
        else {

        }

       if num1 == 3.14 {

       }
            else if num2 == 3.14 {

            }
                else { 
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
                    }  

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

