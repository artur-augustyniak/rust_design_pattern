#[derive(Debug)]
struct Token {
    value: i32
}

pub fn fv_run() {
    println!("-------------------- {} --------------------", file!());

    let incoming_tokens: Vec<Token> = vec![
        Token { value: 5 },
        Token { value: 10 },
        Token { value: 12 },
        Token { value: -3 },
        Token { value: -4 },
        Token { value: 2 },
        Token { value: 0 },
    ];


    let is_odd = |t: &Token| -> bool { t.value % 2 == 0 };
    let is_negative = |t: &Token| -> bool { t.value < 0 };

    let odd_tokens: Vec<Token> = incoming_tokens.into_iter()
        .filter(|t| is_odd(t))
        .collect();

    println!("{:?}", &odd_tokens);


    let neg_tokens: Vec<Token> = odd_tokens.into_iter()
        .filter(|t| is_negative(t))
        .collect();
    println!("{:?}", &neg_tokens);
}