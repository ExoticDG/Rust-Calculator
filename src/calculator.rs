#[allow(dead_code)]
pub fn calculate(sym:String, num1:String, num2:String) -> Result<String, String> {

    let sym = sym.trim();

    let problem:String = format!("{num1} {sym} {num2}");

    println!("calc strings {num1} {num2} {sym}");

    let n1 = if num1=="3.14" || num1.to_lowercase()=="pi" || num1=="𝜋" {
        Ok(std::f64::consts::PI)
    } else {
        num1.trim().parse::<f64>()
    }
    .map_err(|_e| "Err 3.14: First entry must be a number.".to_owned())?;

    let n2 = if num2=="3.14" || num2.to_lowercase()=="pi" || num2=="𝜋"  {
        Ok(std::f64::consts::PI)
    } else {
        num2.trim().parse::<f64>()
    }
    .map_err(|_e| "Err 3.14: Second entry must be a number.".to_owned())?;

    let answer = if sym == "*" {
        n1 * n2
    } else if sym == "/" {
        n1 / n2
    } else if sym == "-" {
        n1 - n2
    } else if sym == "+" {
        n1 + n2
    } else {
        println!("Sorry. This is not a viable problem. Try again.");
        return Err("Err 83254: Not a viable symbol.".to_owned())
    };

    println!("The answer is {}", answer);

    let result = format!("Answer == {} \n Problem == {}", answer, problem);
    Ok(result)
}
