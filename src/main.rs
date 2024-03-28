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
        println!("{}", simple_expression(2, Operator::Exp, "x".to_string(), None));
    }

    if equasion.is_empty() {
        std::process::exit(1);
    }
}
// fn calculate_derivative(eq: String){
//     let mut derivative = String::new();

//     let mut brackets = false;

//     let mut buffor = String::new();
//     let mut numberBuffor = String::new();

//     let mut levelOfBrackets = 0;
//     for c in eq.chars() {
//         if brackets {
//             if c == ')' && levelOfBrackets == 0 {
//                 brackets = false;
//                 calculate_derivative(buffor.clone());
//                 buffor.clear();
//                 continue;
//             }
//             if c == ')' {
//                 levelOfBrackets -= 1;
//             }
//             if c == '(' {
//                 levelOfBrackets += 1;
//             }
//             buffor.push(c);
//         }
//         if c == '(' {
//             brackets = true;
//         }
//         if c == ')' {
//             println!("Error: Unopened bracket");
//             std::process::exit(1);
//         }

//         if c.is_digit(10) {
//             numberBuffor.push(c);
//         }
//         if c == '+' || c == '-'  {
//             if !numberBuffor.is_empty() {
//                 if buffor.is_empty() {
//                     // 0
//                 } else {
//                     calculate_derivative(buffor);
//                     buffor.clear();
//                 }
//                 numberBuffor.clear();
//             }
//             derivative.push(c);
//         }
//     }
// }
fn simple_expression(num: i32, op: Operator, variable: String, power: Option<i32>) -> String{
    match op {
        Operator::Pow => {
            if power.unwrap() == 2 {
                return format!("*{}{}", num*2, variable);
            }
            return format!("*{}{}^{}", num*power.unwrap(), variable, power.unwrap() - 1);
        }
        Operator::Sin => {
            println!("*{}cos({})", num, variable);
        }
        Operator::Cos => {
            println!("*{}sin({})", num, variable);
        }
        Operator::Tan => {
            println!("*{}sec^2({})", num, variable);
        }
        Operator::Log => {
            println!("*{}/{}ln(10)", num, variable);
        }
        Operator::Ln => {
            println!("{}/{}", num, variable);
        }
        Operator::Sqrt => {
            println!("*{}/(2*sqrt({}))", num, variable);
        }
        Operator::Exp => {
            println!("*{}exp({})", num, variable);
        }
    }
    return String::new();
}