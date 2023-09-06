use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("請猜一個數字！");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("祕密數字為：{secret_number}");

    println!("請輸入你猜的數字：");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("讀取該行輸入失敗");


    let guess:u32 = guess.trim().parse().expect("請輸入你猜的數字：");

    println!("你猜的數字：{guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => print!("太小了！"),
        Ordering::Greater=> print!("太大了！"),
        Ordering::Equal => print!("猜中了！")
    }
}
