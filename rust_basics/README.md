# Rust基礎文法 学習プロジェクト

Rustの基礎文法を体系的に学ぶための学習用プロジェクトです。  
各セクションは独立したモジュール（`.rs`ファイル）として実装されており、`cargo run`で全セクションを通して実行できます。

## 実行方法

```bash
cargo run
```

## 構成

```
src/
├── main.rs            # エントリポイント・各セクションの呼び出し
├── structs.rs         # セクション5: 構造体とメソッド
├── enums.rs           # セクション6: 列挙型とパターンマッチング
├── traits.rs          # セクション7: トレイト
├── lifetimes.rs       # セクション8: ライフタイム
├── closures.rs        # セクション9: クロージャとイテレータ
└── error_handling.rs  # セクション10: エラーハンドリング
```

## 学習内容

### セクション1: 変数と型 (`main.rs`)
- `let` による変数宣言（immutable / mutable）
- 型推論と型注釈
- タプル・配列

### セクション2: 関数 (`main.rs`)
- 引数と戻り値
- 式としての関数（セミコロンなしの最終式がreturn値）

### セクション3: 条件分岐・ループ (`main.rs`)
- `if` 式（値を返せる）
- `loop` / `for` / `while`
- イテレータと `enumerate()`

### セクション4: 所有権の基礎 (`main.rs`)
- 所有権の移動（move）
- 参照と借用（`&`）
- Copy型 vs Move型

### セクション5: 構造体とメソッド (`structs.rs`)
- `struct` の定義
- `impl` ブロックによるメソッド定義
- 関連関数（`new()`）とメソッド（`&self`）

### セクション6: 列挙型とパターンマッチング (`enums.rs`)
- データを持つ `enum`（代数的データ型）
- `match` による網羅的なパターンマッチング
- `Option<T>`（nullの代替）
- `Result<T, E>`（エラーハンドリングの基礎）

### セクション7: トレイト (`traits.rs`)
- `trait` の定義とデフォルト実装
- `impl Trait for Type` による明示的実装
- `Box<dyn Trait>`（動的ディスパッチ）
- ジェネリクスとtrait境界（`<T: Trait>`）
- 組み込みtrait（`Debug`, `Display`, `Clone`, `PartialEq`）

### セクション8: ライフタイム (`lifetimes.rs`)
- ダングリング参照とライフタイム検査
- ライフタイム注釈（`'a`）
- 構造体のライフタイム
- ライフタイム省略規則

### セクション9: クロージャとイテレータ (`closures.rs`)
- クロージャの構文と型推論
- 変数のキャプチャ（借用 vs `move`）
- `map`, `filter`, `fold`, `collect` などのイテレータアダプタ
- 遅延評価（終端操作が来るまで実行されない）

### セクション10: エラーハンドリング (`error_handling.rs`)
- `Result<T, E>` の基本操作（`match`, `unwrap_or`, `map`）
- `?` 演算子によるエラー伝播
- カスタムエラー型（`enum` + `Display` + `Debug`）
- `From` トレイトによるエラー型の自動変換
- `Box<dyn Error>` による型消去

## 参考
- [The Rust Programming Language（公式ドキュメント）](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
