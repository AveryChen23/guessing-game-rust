use std::io;
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

    println!("你猜的數字：{guess}")
}
