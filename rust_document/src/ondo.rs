// ondo.rs
use std::io;

// 華氏から摂氏への変換
pub fn kasi_to_sesi(kasi: f64) -> f64 {
    (kasi - 32.0) * 5.0 / 9.0
}

// 摂氏から華氏への変換
pub fn sesi_to_kasi(sesi: f64) -> f64 {
    (sesi * 9.0 / 5.0) + 32.0
}

// 温度変換プログラム本体
pub fn ondo() {
    println!("温度をへんかんするぜ!");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("ミスったわ");

    let choice = choice.trim();

    match choice {
        "1" => {
            println!("華氏を摂氏に変えるぜ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("ごめんミスったわ");

            match input.trim().parse::<f64>() {
                Ok(kasi) => {
                    let sesi = kasi_to_sesi(kasi);
                    println!("華氏{}は摂氏{:.2}", kasi, sesi);
                }
                Err(_) => {
                    println!("有効な数値を入れてな");
                }
            }
        }
        "2" => {
            println!("摂氏を華氏に変えるぜ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("ごめんミスったわ");

            match input.trim().parse::<f64>() {
                Ok(sesi) => {
                    let kasi = sesi_to_kasi(sesi);
                    println!("摂氏{}は華氏{:.2}", sesi, kasi);
                }
                Err(_) => {
                    println!("有効な数字を入れてな");
                }
            }
        }
        _ => {
            println!("無効選択だ。1または2を選択してな");
        }
    }
}
