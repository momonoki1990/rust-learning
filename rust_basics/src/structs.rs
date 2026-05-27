// セクション5: 構造体(struct)とメソッド

pub fn run() {
    println!("--- セクション5: 構造体とメソッド ---");

    // struct でデータ構造を定義
    // implブロックでメソッドを追加する
    let rect = Rectangle::new(10.0, 5.0);
    println!("面積: {}", rect.area());
    println!("周囲: {}", rect.perimeter());
    println!("正方形?: {}", rect.is_square());

    // 別の長方形と比較
    let small = Rectangle::new(3.0, 3.0);
    println!("rectはsmallより大きい?: {}", rect.is_larger_than(&small));

    // TODO(human): Circleの実装
    // 下のコードが動くようにCircle構造体とimplブロックを実装してください
    let circle = Circle::new(7.0);
    println!("\n円の面積: {:.2}", circle.area());
    println!("円の周囲長: {:.2}", circle.circumference());
    println!("半径2倍の円の面積: {:.2}", circle.scale(2.0).area());

    println!();
}

// 長方形の構造体（実装済み・参考にしてください）
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 関連関数(associated function): selfを取らない → Rectangle::new() で呼ぶ
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // メソッド: &self で自身への参照を受け取る
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn is_larger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

// TODO(human): Circle構造体を実装してください
// 以下の3つを実装する:
//   1. struct Circle { ... }            フィールド: radius: f64
//   2. impl Circle の new(radius) -> Circle
//   3. impl Circle の area() -> f64     (π * r^2, π は std::f64::consts::PI)
//   4. impl Circle の circumference() -> f64  (2 * π * r)
//   5. impl Circle の scale(factor: f64) -> Circle  (半径をfactor倍した新しいCircleを返す)

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn scale(&self, factor: f64) -> Circle {
        Circle {
            radius: self.radius * factor,
        }
    }
}
