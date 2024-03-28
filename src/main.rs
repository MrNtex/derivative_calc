#[derive(PartialEq)]
enum Operator {
    Pow,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Sqrt,
    Exp,
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut equasion = String::new();

    if args.len() < 2 {
        println!("Usage: {} <Equasion>", args[0]);
        std::process::exit(1);
    }
    if (args[1] == "-h") || (args[1] == "--help") {
        println!("Usage: {} <Equasion>", args[0]);
        std::process::exit(0);
    }
    if(args[1] == "-e") || (args[1] == "--equasion") {
        if args.len() < 3 {
            println!("Usage: {} -e <Equasion>", args[0]);
            std::process::exit(1);
        }
        equasion = args[2].clone();
        let mut result = split_expression(equasion.clone());

        for i in 0..result.len() {
            println!("{}", result[i].clone());
            println!("{}", calculate_simple_derivative(result[i].clone()));
        }
    }

    if equasion.is_empty() {
        std::process::exit(1);
    }
}

fn split_expression(expression: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut temp = String::new();
    for c in expression.chars() {
        if c == '+' || c == '-' || c == '(' || c== ')' {
            if !temp.is_empty() {
                result.push(temp.clone());
                temp.clear();
            }
            result.push(c.to_string());
        } else {
            temp.push(c);
        }
    }
    if !temp.is_empty() {
        result.push(temp.clone());
    }
    result
}
fn calculate_simple_derivative(expression: String) -> String {
    let mut result = String::new();
    let mut cooficient = String::new();
    let mut power = String::new();
    let mut operator = Operator::Pow;
    let mut seen_operator = false;

    let mut chain_rule = String::new();
    for c in expression.chars(){
        if !seen_operator{
            if c.is_numeric(){
                cooficient.push(c);
            }else if c == 'x' {
                continue;
            }else{
                seen_operator = true;
                match c {
                    'p' => operator = Operator::Pow,
                    's' => operator = Operator::Sin,
                    'c' => operator = Operator::Cos,
                    't' => operator = Operator::Tan,
                    'l' => operator = Operator::Log,
                    'n' => operator = Operator::Ln,
                    'r' => operator = Operator::Sqrt,
                    'e' => operator = Operator::Exp,
                    _ => {}
                }
            
            }
        }
        else{
            if operator == Operator::Pow {
                power.push(c);
            }else{
                chain_rule.push(c);
            }
        }
    }
    println!("Cooficient: {}, power: {}", cooficient, power);
    let mut new_cooficient = 1;
    if !cooficient.is_empty() {
        new_cooficient = cooficient.parse::<i32>().unwrap();
    }
    match operator {
        
        Operator::Pow => {
            let new_power = power.parse::<i32>().unwrap() - 1;

            result.push_str(&(new_cooficient*(new_power+1)).to_string());
            if new_power == 1 {
                result.push_str("x");
            }else{
                result.push_str("x^");
                result.push_str(&new_power.to_string());
            }
        }
        Operator::Sin => {
            result.push_str(&cooficient);
            result.push_str("cos(");
            result.push_str(&chain_rule);
            result.push_str(")");
        }
        Operator::Cos => {
            result.push_str("-");
            result.push_str(&cooficient);
            result.push_str("sin(");
            result.push_str(&chain_rule);
            result.push_str(")");
        }
        Operator::Tan => {
            result.push_str(&cooficient);
            result.push_str("sec^2(");
            result.push_str(&chain_rule);
            result.push_str(")");
        }
        Operator::Log => {
            result.push_str(&cooficient);
            result.push_str("1/");
            result.push_str(&chain_rule);
        }
        Operator::Ln => {
            result.push_str(&cooficient);
            result.push_str("1/");
            result.push_str(&chain_rule);
        }
        Operator::Sqrt => {
            result.push_str(&cooficient);
            result.push_str("1/(2*sqrt(");
            result.push_str(&chain_rule);
            result.push_str(")");
        }
        Operator::Exp => {
            result.push_str(&cooficient);
            result.push_str("exp(");
            result.push_str(&chain_rule);
            result.push_str(")");
        }
    }
    result
}