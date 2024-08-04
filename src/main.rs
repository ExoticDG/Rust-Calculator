slint::include_modules!();

mod calculator;
use calculator::calculate;

#[cfg(any(target_os = "windows", target_os = "linux"))]
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
            
            let ans = match calculate(s.to_string(), n1.to_string(), n2.to_string()) {
                Ok(x) => x,
                Err(e) => e,
            };
            ui.set_result(ans.into());
        }
    });
    let _ = ui.run();
    ui.run()?;

    Ok(())
} 


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