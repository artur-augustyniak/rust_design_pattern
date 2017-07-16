use std::fmt::Debug;
use std::cell::RefCell;

trait Drawable: Debug {
    fn draw(&self);
}

#[derive(Debug)]
struct Font {
    c: char
}

impl Drawable for Font {
    fn draw(&self) {
        print!("{}", self.c);
    }
}


#[derive(Debug)]
struct Paragraph<'a> {
    letters: RefCell<Vec<&'a Drawable>>
}


impl<'a> Paragraph<'a> {
    pub fn insert_font(&self, f: &'a Drawable) {
        self.letters.borrow_mut().push(f);
    }
}


impl<'a> Drawable for Paragraph<'a> {
    fn draw(&self) {
        print!("<p>");
        for f in self.letters.borrow().iter() {
            f.draw();
        }
        println!("</p>");
    }
}

fn client_mock(d:&Drawable){
    d.draw();
    println!("\nend of client mock");
}

pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let f1 = Font { c: 'a' };
    let f2 = Font { c: 'b' };

    let paragraph = Paragraph { letters: RefCell::new(Vec::new()) };
    paragraph.insert_font(&f1);
    paragraph.insert_font(&f1);
    paragraph.insert_font(&f2);
    paragraph.draw();
    println!("-------------------- {} --------------------", "client mock");
    client_mock(&f1);
    client_mock(&f2);
    client_mock(&paragraph);
}