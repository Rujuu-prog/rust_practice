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

// トレイトとは共通の振る舞いを定義するもの
// メソッドのシグニチャー（特徴、固有な特徴）を定義するだけで、処理の中身は実装しない
// 共通の振る舞いを定義したトレイトと、任意の型を紐つけることで、紐づけられた型に対してトレイトに定義されたメソッドの実装を強制できる
// 任意の型とトレイトを紐づけることを、トレイトを実装するという
// 異なる点はあるが、他の言語でいうインターフェースや抽象クラスと似ている
pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn say_shape_name();
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }
        fn say_shape_name() {
            println!("Rectangle!");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }
    
    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }
        fn say_shape_name() {
            println!("Circle!");
        }
    }
}