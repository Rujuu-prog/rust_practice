pub fn test1() {
    println!("test1");
}
fn test2(){
    println!("test2");
}

// structにpubをつけただけでは、構造体はpublicになるけど、fieldはprivateのままになる
// つまり、同一module内からはfieldにアクセスできるけど、外部からはフィールドにアクセスできず、インスタンス化できない
// pub struct TestStruct {
//     v1: i32,
//     v2: i32,
// }

// 外部からインスタンス化するには2つの方法がある
// 1：fieldを全てpublicにする
// pub struct TestStruct {
//     pub v1: i32,
//     pub v2: i32,
// }

// 2：構造体のインスタンスを返す、publicな型関連関数newを定義する（オブジェクト指向のカプセル化のような感じ）
pub struct TestStruct {
    v1: i32,
    v2: i32,
}

impl TestStruct {
    pub fn new(v1: i32, v2: i32) -> Self {
        Self{v1, v2}
    }
    pub fn add(&self){
        println!("test_struct add: {}", self.v1+self.v2);
    }
}