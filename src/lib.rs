//! ライブラリクレートのドキュメント（クレート全体のドキュメントになる）
//! 他のクレートから読み込む時はpublicにする必要がある

/// say_hello関数のドキュメント
pub fn say_hello() {
    println!("Hello, world!");
}

/// **say_goodbye** 関数のドキュメント
/// <p>Markdown形式でも書ける</p>
/// ### 使い方
/// ```
/// fn main() {
///     rust_practice::say_goodbye();
/// }
/// ```
pub fn say_goodbye() {
    println!("Goodbye");
}