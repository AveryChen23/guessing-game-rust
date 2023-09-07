use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let mut min = 1;
    let mut max = 100;
    let secret_number = rand::thread_rng().gen_range(min..=max);
    loop {
        println!("請從{min}到{max}猜一個數字！");

        println!("請輸入你猜的數字：");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行輸入失敗");
    
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你猜的數字：{guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                print!("太小了！");
                min = guess + 1;
            },
            Ordering::Greater=> {
                print!("太大了！");
                max = guess - 1;
            },
            Ordering::Equal => {
                print!("猜中了！");
                break;
            }
        }        
    }

}
