// セクション7: トレイト(trait)

pub fn run() {
    println!("--- セクション7: トレイト ---");

    basic_trait();
    trait_objects();
    generic_functions();
    builtin_traits();

    println!();
}

// ===== traitの定義と実装 =====

// traitは「共通の振る舞い」を定義するインターフェース
trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> String;

    // デフォルト実装: implで上書きしなければこれが使われる
    fn summary(&self) -> String {
        format!("{} (面積: {:.2})", self.describe(), self.area())
    }
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// RectangleにShapeトレイトを実装（参考）
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn describe(&self) -> String {
        format!("長方形({}x{})", self.width, self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn describe(&self) -> String {
        format!("円(半径{})", self.radius)
    }
}

fn basic_trait() {
    println!("\n[traitの基本]");

    let rect = Rectangle {
        width: 4.0,
        height: 3.0,
    };
    let circle = Circle { radius: 5.0 };

    // summary()はデフォルト実装が使われる
    println!("{}", rect.summary());
    println!("{}", circle.summary());
}

// ===== trait object: Box<dyn Trait> =====
// 異なる型を同じVecに入れたいときに使う（実行時に型が決まる）
fn trait_objects() {
    println!("\n[trait object: Box<dyn Shape>]");

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 5.0,
        }),
        Box::new(Circle { radius: 1.0 }),
    ];

    for s in &shapes {
        println!("  {}", s.summary());
    }

    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("合計面積: {:.2}", total);
}

// ===== ジェネリクス + trait境界: <T: Trait> =====
// コンパイル時に型が確定する（静的ディスパッチ）→ 実行時コストなし
fn print_shape_info<T: Shape>(shape: &T) {
    println!("  {}", shape.summary());
}

fn largest_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).fold(0.0_f64, f64::max)
}

fn generic_functions() {
    println!("\n[ジェネリクス + trait境界]");

    let circles = vec![
        Circle { radius: 2.0 },
        Circle { radius: 5.0 },
        Circle { radius: 3.0 },
    ];

    for c in &circles {
        print_shape_info(c);
    }
    println!("最大面積: {:.2}", largest_area(&circles));
}

// ===== 標準ライブラリのtrait =====
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// Displayトレイト: println!("{}", ...) のフォーマットを定義する
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn builtin_traits() {
    println!("\n[組み込みtrait]");

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone(); // Clone trait
    println!("Display: {}", p1); // Display trait
    println!("Debug:   {:?}", p1); // Debug trait (#[derive(Debug)])
    println!("等しい?: {}", p1 == p2); // PartialEq trait
}
