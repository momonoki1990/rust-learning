// セクション9: クロージャとイテレータ

pub fn run() {
    println!("--- セクション9: クロージャとイテレータ ---");

    basic_closures();
    capturing_variables();
    iterator_basics();
    iterator_chaining();
    custom_processing();

    println!();
}

// ===== クロージャの基本 =====
fn basic_closures() {
    println!("\n[クロージャの基本]");

    // 関数と比較
    fn double_fn(x: i32) -> i32 {
        x * 2
    }
    let double_closure = |x: i32| x * 2; // 型推論で省略もできる
    let double_short = |x| x * 2; // さらに短く

    println!("関数:    {}", double_fn(5));
    println!("クロージャ: {}", double_closure(5));
    println!("短縮形:  {}", double_short(5));

    // 複数行クロージャ
    let describe = |n: i32| {
        if n > 0 {
            format!("{} は正", n)
        } else {
            format!("{} は非正", n)
        }
    };
    println!("{}", describe(3));
    println!("{}", describe(-1));
}

// ===== 変数のキャプチャ =====
fn capturing_variables() {
    println!("\n[変数のキャプチャ]");

    let threshold = 10; // クロージャの外の変数

    // クロージャは周囲の変数を「借用」してキャプチャできる
    let is_over = |x: i32| x > threshold; // threshold を借用

    println!("5 > {}? {}", threshold, is_over(5));
    println!("15 > {}? {}", threshold, is_over(15));
    println!("threshold はまだ使える: {}", threshold);

    // move: 所有権ごと取り込む（スレッドに渡すときなどに使う）
    let prefix = String::from("結果");
    let format_result = move |n: i32| format!("{}: {}", prefix, n);
    // println!("{}", prefix); // ← move後は使えない
    println!("{}", format_result(42));
}

// ===== イテレータの基本 =====
fn iterator_basics() {
    println!("\n[イテレータの基本]");

    let numbers = vec![1, 2, 3, 4, 5];

    // map: 各要素を変換 → 新しいイテレータ（まだ実行されない）
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect(); // ← ここで初めて実行される
    println!("2倍: {:?}", doubled);

    // filter: 条件を満たす要素だけ残す
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // fold: 畳み込み（合計・積など）
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("合計: {}", sum);

    // sum / product: よく使う畳み込みはメソッドがある
    let sum2: i32 = numbers.iter().sum();
    println!("合計(sum): {}", sum2);
}

// ===== イテレータのチェーン =====
fn iterator_chaining() {
    println!("\n[イテレータのチェーン]");

    let words = vec!["hello", "world", "rust", "is", "great"];

    // 複数の操作をチェーンする
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3) // 4文字以上
        .map(|w| w.to_uppercase()) // 大文字に
        .collect();
    println!("4文字以上を大文字に: {:?}", result);

    // enumerate: インデックス付きで走査
    for (i, word) in words.iter().enumerate() {
        print!("[{}]{} ", i, word);
    }
    println!();

    // zip: 2つのイテレータを組み合わせる
    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![85, 92, 78];
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(scores.iter()).collect();
    for (name, score) in &pairs {
        println!("  {}: {}", name, score);
    }
}

// ===== 実践的な処理 =====

struct Student {
    name: String,
    score: u32,
}

// TODO(human): students_summary関数を実装してください
// 以下の処理をイテレータチェーンで書いてください:
//   1. 60点以上の学生だけ絞り込む
//   2. "名前: XX点" の形式のStringに変換する
//   3. Vec<String>として返す
// ヒント: filter → map → collect の順に繋げる
//         format!("{}: {}点", s.name, s.score) で文字列を作る
fn students_summary(students: &[Student]) -> Vec<String> {
    students
        .iter()
        .filter(|s| s.score >= 60)
        .map(|s| format!("{}: {}点", s.name, s.score))
        .collect()
}

fn custom_processing() {
    println!("\n[実践的なイテレータ処理]");

    let students = vec![
        Student {
            name: "Alice".to_string(),
            score: 92,
        },
        Student {
            name: "Bob".to_string(),
            score: 55,
        },
        Student {
            name: "Carol".to_string(),
            score: 78,
        },
        Student {
            name: "Dave".to_string(),
            score: 41,
        },
        Student {
            name: "Eve".to_string(),
            score: 88,
        },
    ];

    let summary = students_summary(&students);
    println!("合格者:");
    for s in &summary {
        println!("  {}", s);
    }
    println!("合格者数: {}", summary.len());
}
