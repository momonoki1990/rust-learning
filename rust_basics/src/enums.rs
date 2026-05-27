// セクション6: 列挙型(enum)とパターンマッチング

pub fn run() {
    println!("--- セクション6: 列挙型とパターンマッチング ---");

    // --- 基本的なenum ---
    basic_enum();

    // --- データを持つenum ---
    enum_with_data();

    // --- Option<T>: nullの代わり ---
    option_usage();

    // --- Result<T, E>: エラーハンドリング ---
    result_usage();

    println!();
}

// ===== 基本的なenum =====
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn basic_enum() {
    println!("\n[基本enum]");

    let dir = Direction::North;

    // match: 全バリアントを網羅しないとコンパイルエラー！
    let description = match dir {
        Direction::North => "北へ進む",
        Direction::South => "南へ進む",
        Direction::East => "東へ進む",
        Direction::West => "西へ進む",
    };
    println!("方向: {:?}", Direction::North);
    println!("{}", description);
}

// ===== データを持つenum =====
#[derive(Debug)]
enum Shape {
    Circle(f64),             // 半径
    Rectangle(f64, f64),     // 幅, 高さ
    Triangle(f64, f64, f64), // 3辺
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // ヘロンの公式
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("円(半径{})", r),
            Shape::Rectangle(w, h) => format!("長方形({}x{})", w, h),
            Shape::Triangle(a, b, c) => format!("三角形(辺: {}, {}, {})", a, b, c),
        }
    }
}

fn enum_with_data() {
    println!("\n[データを持つenum]");

    let shapes = vec![
        Shape::Circle(7.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{}: 面積 = {:.2}", shape.describe(), shape.area());
    }
}

// ===== Option<T> =====
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn option_usage() {
    println!("\n[Option<T>]");

    let nums = vec![1, 3, 5, 4, 7];
    match find_first_even(&nums) {
        Some(n) => println!("最初の偶数: {}", n),
        None => println!("偶数は見つからなかった"),
    }

    let empty: Vec<i32> = vec![1, 3, 5];
    // unwrap_or: Noneのときデフォルト値を使う
    let result = find_first_even(&empty).unwrap_or(-1);
    println!("偶数(なければ-1): {}", result);
}

// ===== Result<T, E> =====
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSqrt,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSqrt)
    } else {
        Ok(x.sqrt())
    }
}

fn result_usage() {
    println!("\n[Result<T, E>]");

    let cases = vec![(10.0, 2.0), (5.0, 0.0)];
    for (a, b) in cases {
        match divide(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(MathError::DivisionByZero) => println!("エラー: ゼロ除算"),
            Err(e) => println!("エラー: {:?}", e),
        }
    }

    let sqrt_cases = vec![9.0, -4.0];
    for x in sqrt_cases {
        match safe_sqrt(x) {
            Ok(result) => println!("√{} = {:.2}", x, result),
            Err(MathError::NegativeSqrt) => println!("エラー: {}の平方根は実数でない", x),
            Err(e) => println!("エラー: {:?}", e),
        }
    }
}
