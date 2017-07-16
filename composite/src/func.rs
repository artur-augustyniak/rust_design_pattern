pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let font = |c: char| {
        Box::new(move || {
            print!("{}", c);
        })
    };


    let paragraph = |fonts: Vec<Box<Fn()>>| {
        Box::new(move || {
            print!("<p>");
            for f in fonts.iter() {
                f();
            }
            println!("</p>");
        })
    };


    let f1 = font('a');
    let f2 = font('b');
    println!("simple font print");
    f1();
    f2();
    println!("\nsimple font end");

    let par = paragraph(vec![f1, f2]);
    par();
}
