// 逆ポーランド記法の数式を計算するプログラム
// - 演算子は + - * / %
// - 数値は32bit整数

use std::env;

// 外部モジュールの読み込みの際には、modを使う
mod reverse_polish;

fn main() {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();

    // ポーランド記法に必要な引数値のみ抽出
    let mut values: Vec<&str> = Vec::new();

    for i in 1..args.len() {
        values.push(&args[i]);
    }

    // 計算
    let result = reverse_polish::eval(&mut values);

    println!("calculation result is {}", result);
}
