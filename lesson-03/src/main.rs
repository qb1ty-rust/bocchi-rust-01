#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
use std::num::Wrapping;

fn main() {
    q1();
    q2();
    q3();
    q4();
    q5();
    q6();
    q7();
}

fn q1() {
    println!("What is equal 7 * 8?");

    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .unwrap();

    if input.trim() == "56" {
        println!("Correct");
    } else {
        println!("Incorrect");
    }
}

fn q2() {
    let x: u64 = 4_294_967_296;
    let y = x as u32;

    if x == y as u64 {
        println!("x equal y");
    } else {
        println!("x does not equal y");
    }
}

fn q3() {
    let a: f32 = 5.5028236;

    println!("{a}");
}

fn q4() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let c: f64 = 0.3;
    let error_margin: f64 = f64::EPSILON;

    if (a + b - c).abs() < error_margin {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn q5() {
    let mut count: Wrapping<i8> = Wrapping(0i8);
    let mut counter: i8 = 0;

    loop {
        println!("{counter}");
        count += 1;
        count += Wrapping(1i8);
        counter = counter.checked_add(1).expect("fail!");
    }
}

fn q6() {
    let str = "Hello!";
    let str2 = "Привет!";

    println!("String {} has {} length", str, str.len());
    println!("String {} has {} length", str2, str2.chars().count());
}

fn call_me(n: u64, _: i32, c: u32) -> u64 {
    println!("{c}");
    n * 2
}

fn q7() {
    let one: i32 = 1;
    let c: i8 = -3;
    let n = call_me(one as _, 3, c as _);

    println!("{n}");
}