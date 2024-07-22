
// use std::{env::{self,}, io,};
slint::include_modules!();

/* 
fn os() {
    let mut ostype = String::new();
    println!(
        "Hello! It looks like you are using the {} os. Is this correct? (Y/N)",
        env::consts::OS
    );

    ostype.clear();
    io::stdin().read_line(&mut ostype).unwrap();
    ostype = ostype.trim().to_string();

    if ostype.to_lowercase() == "y" {
        if cfg!(windows) {
            if env::consts::OS == "windows" {
                println!("Thanks for confirming.");
            }
            } else if cfg!(unix) {
                if env::consts::OS == "linux" {
                    println!("Thanks for confirming.");
                } else {
                    panic!("Exit Code 837 There is a problem with OS detection.");
                }
            }
        
    } else if ostype.to_lowercase() == "n" {
        if env::consts::OS == "windows" {
            println!("This is Windows mate. Stop trying to be funny. Your not.");
        } else if cfg!(unix) {
            if env::consts::OS == "linux" {
                println!("This is Linux mate. Stop trying to be funny. Your not.");
            } else {
                panic!("Exit Code 837. There is a problem with OS detection.");
            }
        }
    }
}
*/

fn main() -> Result<(), slint::PlatformError> {
 //   os();
    let ui = AppWindow::new()?;
    ui.on_calc({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let n1 = ui.get_num1ui();
            let n2 = ui.get_num2ui();
            let s = ui.get_symui();
            
            let ans = calculate(s.to_string(), n1.to_string(), n2.to_string());
            ui.set_result(ans.into());
        }
        

    });
    ui.run();

    Ok(())
} 


fn calculate(sym:String, num1:String, num2:String) -> String {
    let mut answer:f64 = 0.0;
    let problem:String = format!("{num1} {sym} {num2}");

    println!("calc strings {num1} {num2} {sym}");

    let n1 = if num1=="3.14" || num1.to_lowercase()=="pi" || num1=="𝜋" {
        std::f64::consts::PI
    } else {
        num1.trim().parse::<f64>().unwrap()
    };

    let n2 = if num2=="3.14" || num2.to_lowercase()=="pi" || num2=="𝜋"  {
        std::f64::consts::PI
    } else {
        num2.trim().parse::<f64>().unwrap()
    };


    if sym == "*" {
        answer = n1 * n2;
        println!("The answer is {}", answer);
    } else if sym == "/" {
        answer = n1 / n2;
        println!("The answer is {}", answer);
    } else if sym == "-" {
        answer = n1 - n2;
        println!("The answer is {}", answer);
    } else if sym == "+" {
        answer = n1 + n2;
        println!("The answer is {}", answer);
    } else {
        println!("Sorry. This is not a viable problem. Try again.")
    };

    let result = format!("Answer == {} \n Problem == {}", answer, problem);
    result
}
