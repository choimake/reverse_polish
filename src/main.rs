// 逆ポーランド記法の数式を計算するプログラム
// - 演算子は + - * / %
// - 数値は32bit整数

use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// deriveは「引き出す」という意味
// この場合は、Clapの処理をOpsに継承している
#[derive(Clap)]
// clapで利用する初期値を指定している。こういう書き方ができるらしい。
#[clap(
    name = "reverse polish",
    version = "1.0.0",
    author = "choimake",
    about = "sample RPN calculator"
)]
struct Opts {
    /// Sets the level of visibility
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}
// 外部モジュールの読み込みの際には、modを使う
mod reverse_polish;

fn main() {
    // 引数を受け取る
    let opts = Opts::parse();

    // ファイル名が指定されているか判定
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // 標準入力
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, _verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        // 計算
        let result = reverse_polish::eval(&line);
        println!("calculation result is {}", result);
    }
}
