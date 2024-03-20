use uncertainty::*;
use colored::Colorize;
use regex::Regex;
use std::io::{self, Write};

fn main() {
    //uncertainty::main();
    loop {
        let prompt = "uncertainty>".truecolor(250, 200, 152);
        print!("{}",prompt);
        io::stdout().flush().unwrap();

        let re = Regex::new(
            r"\((\d+(\.\d+)?),(\d+(\.\d+)?)\)\s*([+\-*/])\s*\((\d+(\.\d+)?),(\d+(\.\d+)?)\)",
        )
        .unwrap(); // (f64,f64)+(f64,f64)

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.retain(|c| !c.is_whitespace()); // remove white space
        match input.as_ref() {
            "help" => {
                println!("Enter an expression to perform uncertainty arithmetic.");
                println!("UncertainValues are entered as (value,uncertainty).");
                println!("Supported operations are +,-,*,/ and white space is ignored.");
                println!("For example:");
                println!("(3.5,1.0) + (1.0,0.5)");
                println!("(-2,3) * (4,2.3)");
            }
            "bye" | "quit" | "exit" => break,
            _ => {
                if let Some(captures) = re.captures(&input) {
                    let value1: f64 = captures[1].parse().unwrap();
                    let uncertainty1: f64 = captures[3].parse().unwrap();
                    let uv1 = UncertainValue::new(value1, uncertainty1);

                    let value2: f64 = captures[6].parse().unwrap();
                    let uncertainty2: f64 = captures[8].parse().unwrap();
                    let uv2 = UncertainValue::new(value2, uncertainty2);

                    let operator = captures[5].chars().next().unwrap();                    
                    match operator {
                        '+' => {                            
                            let result = uv1 + uv2;
                            println!("({},{})", result.value, result.uncertainty);
                        }
                        '-' => {
                            let result = uv1 - uv2;
                            println!("({},{})", result.value, result.uncertainty);
                        }
                        '*' => {
                            let result = uv1 * uv2;
                            println!("({},{})", result.value, result.uncertainty);
                        }
                        '/' => {
                            let result = uv1 / uv2;
                            println!("({},{})", result.value, result.uncertainty);
                        }
                        _ => println!("Invalid operator. Valid operators are [+,-,*,/]."),
                    }
                }
            }
        }
    }
}


