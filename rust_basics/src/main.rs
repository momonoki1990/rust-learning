// Rust基本文法 学習プロジェクト
// 各セクションを順番に実装してみましょう！

mod structs;
mod enums;
mod traits;
mod lifetimes;
mod closures;
mod error_handling;

fn main() {
    println!("=== Rust基本文法 学習 ===\n");

    // --- セクション1: 変数と型 ---
    section1_variables();

    // --- セクション2: 関数 ---
    section2_functions();

    // --- セクション3: 条件分岐・ループ ---
    section3_control_flow();

    // --- セクション4: 所有権の基礎 ---
    section4_ownership();

    // --- セクション5: 構造体とメソッド ---
    structs::run();

    // --- セクション6: 列挙型とパターンマッチング ---
    enums::run();

    // --- セクション7: トレイト ---
    traits::run();

    // --- セクション8: ライフタイム ---
    lifetimes::run();

    // --- セクション9: クロージャとイテレータ ---
    closures::run();

    // --- セクション10: エラーハンドリング ---
    error_handling::run();
}

// ===== セクション1: 変数と型 =====
fn section1_variables() {
    println!("--- セクション1: 変数と型 ---");

    // let で変数宣言。Rustはデフォルトで不変(immutable)
    let x = 5;
    println!("x = {}", x);

    // mut をつけると可変(mutable)になる
    let mut count = 0;
    count += 1;
    println!("count = {}", count);

    // 型を明示することもできる
    let pi: f64 = 3.14159;
    let greeting: &str = "こんにちは、Rust！";
    println!("pi = {}, greeting = {}", pi, greeting);

    // タプル: 異なる型をまとめられる
    let person: (&str, u32) = ("Alice", 30);
    println!("名前: {}, 年齢: {}", person.0, person.1);

    // 配列: 同じ型・固定長
    let numbers = [1, 2, 3, 4, 5];
    println!("配列の合計: {}", numbers.iter().sum::<i32>());

    println!();
}

// ===== セクション2: 関数 =====
fn section2_functions() {
    println!("--- セクション2: 関数 ---");

    // 関数呼び出し
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // 文字列を返す関数
    let msg = greet("世界");
    println!("{}", msg);

    // TODO(human): describe_number関数を実装してください
    // ヒント: i32を受け取り、正/負/ゼロを示す文字列を返す
    // 例: describe_number(5) -> "5 is positive"
    //     describe_number(-3) -> "-3 is negative"
    //     describe_number(0) -> "0 is zero"
    let description = describe_number(42);
    println!("{}", description);
    let description = describe_number(-7);
    println!("{}", description);

    println!();
}

// 引数と戻り値のある関数
fn add(a: i32, b: i32) -> i32 {
    a + b // Rustでは最後の式がreturn値（セミコロンなし）
}

fn greet(name: &str) -> String {
    format!("こんにちは、{}！", name)
}

// TODO(human): ここにdescribe_number関数を実装してください
fn describe_number(n: i32) -> String {
    if n > 0 {
        format!("{} is positive", n)
    } else if n < 0 {
        format!("{} is negative", n)
    } else {
        "0 is zero".to_string()
    }
}

// ===== セクション3: 条件分岐・ループ =====
fn section3_control_flow() {
    println!("--- セクション3: 条件分岐・ループ ---");

    // if/else (式として値を返せる！)
    let temperature = 0;
    let weather = if temperature > 30 {
        "暑い"
    } else if temperature > 20 {
        "快適"
    } else {
        "涼しい"
    };
    println!("気温{}度は{}", temperature, weather);

    // loop: 無限ループ + break で値を返す
    let mut i = 0;
    let found = loop {
        if i * i > 50 {
            break i;
        }
        i += 1;
    };
    println!("50を超える最初の平方数の平方根: {}", found);

    // for: イテレータを使う
    print!("1から5: ");
    for n in 1..=5 {
        print!("{} ", n);
    }
    println!();

    // Vec(動的配列)とfor
    let fruits = vec!["リンゴ", "バナナ", "オレンジ"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}番目: {}", i + 1, fruit);
    }

    println!();
}

// ===== セクション4: 所有権の基礎 =====
fn section4_ownership() {
    println!("--- セクション4: 所有権の基礎 ---");

    // 所有権の移動(move): String はmoveされる
    let s1 = String::from("hello");
    let s2 = s1; // s1の所有権がs2に移動
    // println!("{}", s1); // ← コンパイルエラー！s1はもう使えない
    println!("s2 = {}", s2);

    // 参照(&): 所有権を渡さずに借用(borrow)
    let s3 = String::from("world");
    let length = calculate_length(&s3);
    println!("\"{}\"の長さ: {}", s3, length); // s3はまだ使える！

    // Copy型 (i32, f64, bool など): moveではなくコピーされる
    let a = 42;
    let b = a; // i32はCopyなので、aもbも使える
    println!("a = {}, b = {}", a, b);

    println!();
    println!("=== おめでとうございます！基礎完了 ===");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
