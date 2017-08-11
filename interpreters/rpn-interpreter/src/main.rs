extern crate rpn_interpreter;

use std::collections::HashMap;

type ASTResult = Result<Box<Expression>, String>;

trait Expression {
    fn interpret_with(&self, vars: &HashMap<String, i32>) -> i32;
}


struct Operator {
    left: Box<Expression>,
    right: Box<Expression>,
    operation: Box<Fn(i32, i32) -> i32>
}

impl Expression for Operator {
    fn interpret_with(&self, vars: &HashMap<String, i32>) -> i32 {
        (self.operation)(
            self.left.interpret_with(vars),
            self.right.interpret_with(vars)
        )
    }
}


struct Variable {
    token: String
}

impl Expression for Variable {
    fn interpret_with(&self, vars: &HashMap<String, i32>) -> i32 {
        match vars.get(&self.token) {
            Some(number) => *number,
            None => panic!("Unresolved variable binding")
        }
    }
}


struct Parser {
    ast: Box<Expression>
}

impl Parser {
    pub fn new(exp: &str) -> ASTResult {
        let mut stack: Vec<Box<Expression>> = Vec::new();
        for t in exp.split(" ") {
            match t {
                "+" => {
                    let l = Parser::pop_operand(&mut stack, exp)?;
                    let r = Parser::pop_operand(&mut stack, exp)?;
                    Parser::push_operation(&mut stack, l, r, Box::new(|x, y| x + y));
                }
                "-" => {
                    let r = Parser::pop_operand(&mut stack, exp)?;
                    let l = Parser::pop_operand(&mut stack, exp)?;
                    Parser::push_operation(&mut stack, l, r, Box::new(|x, y| x - y));
                }
                _ => {
                    stack.push(
                        Box::new(Variable { token: t.to_string() })
                    );
                }
            }
        }

        //invariant for proper input expression
        if 1 != stack.len() {
            let msg = format!("({}) is non parsable RPN expression, too many variables", exp);
            return Err(msg.to_string());
        }

        if let Some(exp) = stack.pop() {
            Ok(Box::new(Parser { ast: exp }))
        } else {
            Err("Unknown error while stack invariant holds".to_string())
        }
    }


    fn push_operation(stack: &mut Vec<Box<Expression>>,
                      left: Box<Expression>,
                      right: Box<Expression>,
                      op: Box<Fn(i32, i32) -> i32>
    ) {
        stack.push(
            Box::new(
                Operator {
                    left: left,
                    right: right,
                    operation: op
                })
        );
    }

    fn pop_operand(stack: &mut Vec<Box<Expression>>, exp: &str) -> ASTResult {
        let msg = format!("({}) is non parsable RPN expression, too few variables", exp);
        stack.pop().ok_or(msg.to_owned())
    }
}


impl Expression for Parser {
    fn interpret_with(&self, vars: &HashMap<String, i32>) -> i32 {
        self.ast.interpret_with(vars)
    }
}


fn main() {
    println!("-------------------- {} --------------------", file!());
    let expr = "aw x z - +";
    let vars: HashMap<String, i32> = [
        ("aw".to_string(), 5),
        ("x".to_string(), 10),
        ("z".to_string(), 42)
    ].iter().cloned().collect();

    let p = Parser::new(expr);
    match p {
        Ok(parser) => {
            //Parsable::interpret_with(p, &vars)
            let result = parser.interpret_with(&vars);
            println!("expected -27 actual {}", result);
        }
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }
}