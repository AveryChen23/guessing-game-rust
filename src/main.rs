use std::io;

fn main() {
    println!("請猜一個數字！");
    println!("請輸入你猜的數字：");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("讀取該行輸入失敗");

    println!("你猜的數字：{guess}")
}
