// セクション8: ライフタイム(lifetime)

pub fn run() {
    println!("--- セクション8: ライフタイム ---");

    why_lifetimes();
    lifetime_annotations();
    lifetime_in_struct();
    lifetime_elision();

    println!();
}

// ===== なぜライフタイムが必要か =====
fn why_lifetimes() {
    println!("\n[なぜライフタイムが必要か]");

    // 参照は「元データより長く生きられない」
    let result;
    {
        let s = String::from("hello");
        result = &s; // s への参照
        println!("スコープ内: {}", result); // ← ここはOK
        // s はここでドロップ（メモリ解放）される
    }
    // println!("{}", result); // ← コンパイルエラー！s はもう存在しない
    // ライフタイム検査がこのバグをコンパイル時に防ぐ

    println!("ライフタイム検査: ダングリング参照をコンパイル時に防止");
}

// ===== ライフタイム注釈 =====

// 'a は「この参照たちは少なくとも同じ期間有効」という制約を表す
// 戻り値の参照が x か y のどちらかわからないため、明示が必要
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn first_word(s: &str) -> &str {
    for (i, &bytes) in s.as_bytes().iter().enumerate() {
        if bytes == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn lifetime_annotations() {
    println!("\n[ライフタイム注釈]");

    let s1 = String::from("長い文字列です");
    let result;
    {
        let s2 = String::from("短い");
        result = longest(s1.as_str(), s2.as_str());
        println!("longer: {}", result);
    }

    let sentence = String::from("hello world from rust");
    let word = first_word(&sentence);
    println!("最初の単語: {}", word);
}

// ===== 構造体のライフタイム =====

// 構造体が参照を持つ場合、ライフタイムの明示が必要
// 「ImportantExcerptは、参照している文字列より長く生きられない」
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce(&self) -> &str {
        self.part
    }
}

fn lifetime_in_struct() {
    println!("\n[構造体のライフタイム]");

    let novel = String::from("Call me Ishmael. Some years ago...");

    // novel の最初の文を参照として持つ
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("抜粋: {}", excerpt.announce());
    // excerpt は novel より先にドロップされるのでOK
}

// ===== ライフタイム省略規則 =====
// コンパイラが自動推論できる場合は省略できる

// 省略あり（コンパイラが 'a を補完してくれる）
fn first_char(s: &str) -> &str {
    &s[..1]
}

// 上と同じ意味（明示版）
fn first_char_explicit<'a>(s: &'a str) -> &'a str {
    &s[..1]
}

fn lifetime_elision() {
    println!("\n[ライフタイム省略規則]");

    let s = String::from("Rust");
    println!("省略あり: {}", first_char(&s));
    println!("明示版:   {}", first_char_explicit(&s));
    println!("→ 引数が1つの場合、戻り値のライフタイムは自動で一致する");
}
