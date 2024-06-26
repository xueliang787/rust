use std::io;

fn main() {
    let mut guess_number = String::new();
    io::stdin().read_line(&mut guess_number).expect("error");
    println!("this number is {}", guess_number)
}