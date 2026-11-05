/* First script on the rust */
use std::io;

const C: f32 = 32.0;

fn c_to_f(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) + C
}

fn f_to_c(temp: f32) -> f32 {
    (temp - C) * (5.0 / 9.0)
}

fn convert(temp: f32, n_choice: u8) -> Option<f32> {
    match n_choice {
        1 => Some(f_to_c(temp)),
        2 => Some(c_to_f(temp)),
        _ => None
    }
}

fn main() {
    println!("Введите температуру \n(1) с F на C. \n(2) с C на F");

    let mut user_choice: String = String::new();

    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice: u8 = user_choice
        .trim()
        .parse::<u8>()
        .expect("Введите число");

    println!("Теперь введите значение температуры:");

    let mut temperature: String = String::new();

    io::stdin().read_line(&mut temperature).unwrap();

    let t_choice: f32 = temperature
        .trim()
        .parse::<f32>()
        .expect("Введите число");

    match convert(t_choice, n_choice) {
        Some(result) => println!("Вот и температура {result}"),
        None => println!("Что то не понял, ты точно 1 или 2 ввел?")
    }
}