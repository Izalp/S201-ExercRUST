use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut opcao = String::new();
    let x: i32;
    let y: i32;
    let total: i32;

    io::stdin()
        .read_line(&mut num1)
        .expect("failed to read line");
    io::stdin()
        .read_line(&mut num2)
        .expect("failed to read line");

    println!("Escolha uma das opções: + ou *");
    io::stdin()
        .read_line(&mut opcao)
        .expect("failed to readline");

    x = num1.trim().parse().unwrap();
    y = num2.trim().parse().unwrap();

    if opcao.trim() == "+" {
        total = x + y;
        println!("Soma {} + {} = {}", x, y, total)
    } else {
        total = x * y;
        println!("Multiplicacao {} * {} = {}", x, y, total)
    }
}
