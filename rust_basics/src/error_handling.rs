// セクション10: エラーハンドリング

use std::fmt;
use std::num::ParseIntError;

pub fn run() {
    println!("--- セクション10: エラーハンドリング ---");

    result_basics();
    question_mark_operator();
    custom_errors();
    error_conversion();

    println!();
}

// ===== Result の基本 =====
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("ゼロ除算エラー"))
    } else {
        Ok(a / b)
    }
}

fn result_basics() {
    println!("\n[Result の基本]");

    // match で丁寧に処理
    match divide(10.0, 2.0) {
        Ok(v) => println!("10 / 2 = {}", v),
        Err(e) => println!("エラー: {}", e),
    }

    // unwrap_or: エラーならデフォルト値
    let result = divide(5.0, 0.0).unwrap_or(0.0);
    println!("5 / 0 (デフォルト0): {}", result);

    // unwrap_or_else: エラーならクロージャを実行
    let result = divide(5.0, 0.0).unwrap_or_else(|e| {
        println!("  エラー発生: {}", e);
        -1.0
    });
    println!("5 / 0 (エラー時-1): {}", result);

    // map: Ok のときだけ変換する
    let doubled = divide(10.0, 2.0).map(|v| v * 2.0);
    println!("(10/2)*2 = {:?}", doubled);
}

// ===== ? 演算子 =====
// ? は「Ok なら値を取り出す、Err なら即 return Err」の糖衣構文
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?; // parse失敗 → 即 return Err
    Ok(n * 2)
}

fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = parse_and_double(a)?; // 失敗 → 即 return Err
    let y = parse_and_double(b)?; // 失敗 → 即 return Err
    Ok(x + y)
}

fn question_mark_operator() {
    println!("\n[? 演算子]");

    println!("{:?}", parse_and_double("21")); // Ok(42)
    println!("{:?}", parse_and_double("abc")); // Err(...)
    println!("{:?}", parse_and_add("10", "20")); // Ok(60)
    println!("{:?}", parse_and_add("10", "xx")); // Err(...)
}

// ===== カスタムエラー型 =====
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
    NotFound(String),
}

// Display: ユーザー向けのエラーメッセージ
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "パース失敗: {}", e),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "範囲外: {} ({}〜{}の範囲で指定)", value, min, max)
            }
            AppError::NotFound(key) => write!(f, "見つかりません: {}", key),
        }
    }
}

// From トレイト: ParseIntError → AppError に自動変換（? 演算子が使えるようになる）
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// TODO(human): parse_age関数を実装してください
// 文字列を受け取り、1〜120の範囲の年齢として解析する
// 処理の流れ:
//   1. s.trim().parse::<i32>()? でパース（?でParseIntError→AppErrorに自動変換）
//   2. 1〜120の範囲チェック → 範囲外なら Err(AppError::OutOfRange{...}) を返す
//   3. 正常なら Ok(age) を返す
fn parse_age(s: &str) -> Result<i32, AppError> {
    let age = s.trim().parse::<i32>()?;
    if age < 0 || age > 120 {
        Err(AppError::OutOfRange {
            value: age,
            min: 0,
            max: 120,
        })
    } else {
        Ok(age)
    }
}

fn custom_errors() {
    println!("\n[カスタムエラー型]");

    let cases = vec!["25", "abc", "0", "150", " 30 "];
    for s in cases {
        match parse_age(s) {
            Ok(age) => println!("  \"{}\" → 年齢: {}", s, age),
            Err(e) => println!("  \"{}\" → エラー: {}", s, e),
        }
    }
}

// ===== Box<dyn Error>: 複数のエラー型を混在させる =====
fn error_conversion() {
    println!("\n[Box<dyn Error>]");

    // Box<dyn Error> は「何らかのエラー」を表す型消去
    // 複数の異なるエラー型を返す可能性がある場合に使う
    let result: Result<i32, Box<dyn std::error::Error>> = (|| {
        let n: i32 = "42".parse()?; // ParseIntError
        Ok(n + 1)
    })();
    println!("Box<dyn Error>の例: {:?}", result);
}
