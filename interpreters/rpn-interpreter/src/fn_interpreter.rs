use std::collections::HashMap;


fn interpret_with(expr: &str, env: &HashMap<String, i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let tokens: Vec<i32> = expr.split(" ").map(|t|
        match t {
            "+" => {
                let l = stack.pop().expect("To few variables");
                let r = stack.pop().expect("To few variables");
                let sub = l + r;
                stack.push(sub);
                sub
            }
            "-" => {
                let r = stack.pop().expect("To few variables");
                let l = stack.pop().expect("To few variables");
                let sub = l - r;
                stack.push(sub);
                sub
            }
            _ => {
                let n = *(env.get(t).expect("Unresolved variable binding"));
                stack.push(n);
                n
            }
        }
    ).collect();

    //invariant for proper input expression
    if 1 != stack.len() {
        panic!("non parsable RPN expression, too many variables");

    }

    *(tokens.last().expect("Empty result"))
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let expr = "a x - z +";
    let vars: HashMap<String, i32> = [
        ("a".to_string(), 2),
        ("x".to_string(), 3),
        ("z".to_string(), 5)
    ].iter().cloned().collect();

    println!("Expected 4 actual {}", interpret_with(expr, &vars));

    let expr = "a x z - +";
    let vars: HashMap<String, i32> = [
        ("a".to_string(), 5),
        ("x".to_string(), 10),
        ("z".to_string(), 42)
    ].iter().cloned().collect();

    println!("Expected -27 actual {}", interpret_with(expr, &vars));


    let expr = "a b c - +";
    let vars: HashMap<String, i32> = [
        ("a".to_string(), 5),
        ("b".to_string(), 5),
        ("c".to_string(), 10),
    ].iter().cloned().collect();

    println!("Expected 0 actual {}", interpret_with(expr, &vars));
}