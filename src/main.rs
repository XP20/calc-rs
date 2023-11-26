use std::ops::Not;
use std::io::stdin;

fn precendence(ch: char) -> i32 {
    match ch {
        '^' => return 3,
        '/' | '*' => return 2,
        '+' | '-' => return 1,
        _ => return -1,
    }
}

fn infix_to_postfix(expression: String) -> String {
    let mut res = String::new();
    let mut stack: Vec<char> = vec![];
    for ch in expression.chars() {
        // If ch is num or var, push onto res string
        if ch.is_alphanumeric() { res.push(ch); }
        // If ch is start parentheses, push onto stack
        else if ch == '(' { stack.push(ch); }
        // If ch is end parentheses, put parentheses operators onto string
        else if ch == ')' {
            while stack.last().unwrap() != &'(' {
                let last = stack.pop().unwrap();
                res.push(' ');
                res.push(last);
            }
            stack.pop().unwrap();
        }
        // Ch is operator, put lesser precendence operators from stack before ch
        else {
            while stack.is_empty().not() &&
                precendence(ch) <= precendence(*stack.last().unwrap()) {
                res.push(' ');
                res.push(*stack.last().unwrap());
                stack.pop().unwrap();
            }
            res.push(' ');
            stack.push(ch);
        }
    }
   
    // Append remaining operators onto res string
    while stack.is_empty().not() {
        res.push(' ');
        res.push(*stack.last().unwrap());
        stack.pop().unwrap();
    }

    return res;
}

fn eval_postfix(postfix: String) -> String {
    let mut stack: Vec<String> = Vec::new();
    for el in postfix.split_whitespace() {
        if el.parse::<f32>().is_ok() { stack.push(el.to_string()); }
        else {
            let num1: String;
            let num2: String;
            if stack.len() > 1 {
                num2 = stack.pop().unwrap();
                num1 = stack.pop().unwrap();
            } else {
                num1 = "0".to_string();
                num2 = stack.pop().unwrap();
            }
            if num1.parse::<f32>().is_err() || 
               num1.parse::<f32>().is_err() {
                stack.push(format!("{}{}{}", num1, el, num2));
            } else {
                let f1 = num1.parse::<f32>().unwrap();
                let f2 = num2.parse::<f32>().unwrap();
                match el { 
                    "^" => stack.push(f32::powf(f1, f2).to_string()),
                    "/" => stack.push((f1 / f2).to_string()),
                    "*" => stack.push((f1 * f2).to_string()),
                    "+" => stack.push((f1 + f2).to_string()),
                    "-" => stack.push((f1 - f2).to_string()),
                    _ => panic!("{}", el),
                }
            }
        }
    }
   
    let res = stack.join("");
    return res;
}

fn main() {
    let greet: String = String::from("Simple rust calc :3");
    let query: String = String::from("Expression: ");
    println!("{}", greet);

    loop {
        println!("{}", query);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.replace(&[' ', '\n'][..], "");

        if input == "exit" { std::process::exit(0); }

        let postfix = infix_to_postfix(input);
        let res = eval_postfix(postfix);
        println!("\n{}\n", res);
    }
}
