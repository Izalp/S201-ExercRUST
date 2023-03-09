use std::io;

pub fn main() {
    let mut array: [i32; 10] = [0; 10];
    let mut num = String::new();
    let num1: i32;

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read line");
    num1 = num.trim().parse().unwrap();

    preenche(&mut array, num1);
}

fn preenche(array: &mut [i32], x: i32) {
    let mut cont: i32 = 0;
    for n in 0..array.len() {
        array[n] = cont * x;
        cont += 1;
        println!("{}", array[n]);
    }
}
