use std::rc::Rc;
use rand::Rng;
use test_module::sub_module1;
use rust_lesson::sample_trait::{Rectangle, Circle, Shape};
// test_moduleの下位の要素をまとめてimportしたりもできる
// use test_module::{sub_module1, sub_module2};
// 全て取得するなら*も使える
// use test_module::*;

const B: i32 = 2;

fn main() {
    let a: i32 = 1;

    println!("{}", a);
    // a = 2;
    let mut b: i32 = 2;
    println!("{}", b);
    b = 3;
    println!("{}", b);
    let d: i32;
    d = 1;
    println!("{}", d);
    let d: &str = "string";
    println!("{}", d);
    println!("{}", B);
    const A: &str = "const";
    println!("{}", A);

    let e = 3;
    let f = 2.0;
    let g: u32 = 1;
    let minus_calc_a = e as f64 + f;
    println!("{}", minus_calc_a);

    // 論理型
    // true, false
    let h: bool = 1 == 2;
    println!("{}", h);

    // タプル
    let tuple: (i32, &str, bool) = (2, "aa", false);
    let tuple2: (&str, i32, bool) = ("aa", 2, false);
    println!("{:?}", tuple);
    println!("{:?}", tuple2);

    let tuple_0 = tuple.0;
    println!("{}", tuple_0);
    let (t0, t1, _) = tuple2;
    println!("{}", t0);

    // 空のタプル。何も返さない関数の返り値とかで使う？
    let unit = ();

    // 配列　型が全て同じ必要あり
    let list: [i32; 3] = [0, 1, 2];
    let list2: [i32; 100] = [1; 100];
    println!("{:?}", list);

    let list_0 = list[0];
    println!("{}", list_0);

    let [l1, l2, l3] = list;
    // &は参照
    // 指定のstartとendは省略できる。&list[..2]や&list[0..]、&list[..]など
    let list3: &[i32] = &list[0..2];
    println!("{:?}", list3);
    let list4: &[i32] = &list[0..=2];
    println!("{:?}", list4);

    // ベクタ型、配列とは違い、要素数を変えることができる。型は全て同じである必要あり。
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: Vec<i32> = vec![1; 100];

    let mut empty_vec = Vec::new();
    empty_vec.push(1);
    empty_vec.push(2);
    println!("{:?}", empty_vec);

    let pop_vec = empty_vec.pop();
    println!("{:?}", pop_vec);
    println!("{:?}", empty_vec);

    let vec_target = empty_vec[0];
    println!("{}", vec_target);
    // 要素が存在しなくても、えらーではなくNoneがかえる
    let get_vec = empty_vec.get(100);
    println!("{:?}", get_vec);

    let slice_vec = &empty_vec[..1];
    println!("{:?}", slice_vec);

    // 文字型
    let c1: char = 'a';
    let c2 = '@';
    let c3: char = '😊';
    println!("{}", c3);

    // 文字列型
    // 後から変更できないやつは
    let s1: &str = "ああ";
    println!("{}", s1);
    // 後から変更できるやつ
    let s2 = String::from("python");
    let s3: String = "java".to_string();

    let mut s4: String = String::from("Foo");
    s4.push_str(", Bar");
    println!("{}", s4);

    println!("{}", s4 + ", golang");

    let s5: String = format!("{}{}", s1, s2);
    println!("{}", s5);

    say_hello();

    let add_result = add(2, 3);
    let hello_result = say_hello();
    // ブロック
    {
        // ブロックの内側で定義された変数は外で使えない
        let inner_x: i32 = 2;
        println!("{}", inner_x);
        // ブロックの外側はOK
        println!("{}", add_result);
    }
    // println!("{}", inner_x);NG

    // シャドーイング
    let shadowing: i32 = 10;
    println!("{}", shadowing);
    {
        let shadowing: i32 = 5;
        println!("{}", shadowing);
    }
    println!("{}", shadowing);

    // ブロックは式
    let siki_block: i32 = {
        100
    };
    println!("{}", siki_block);

    // if
    let if_x: i32 = 15;
    if if_x > 0 {
        println!("OK!");
    }

    if if_x > 0 && if_x < 10 {
        println!("0<x<10");
    }

    if if_x > 0 || if_x < 10 {
        println!("0<x or x<10");
    }

    if if_x > 0 && if_x <= 10 {
        println!("first condition");
    } else if if_x > 11 && if_x <= 20 {
        println!("second condition");
    } else {
        println!("else")
    }

    // ifは式
    let if_result: i32 = if if_x > 10 {
        11
    } else {
        12
    };
    println!("{}", if_result);

    practice_match(0);

    practice_loop();

    practice_while();

    practice_for();

    let fb_while_result = fizzbuzz_while(16);
    println!("{}", fb_while_result);

    let fb_for_match = fizzbuzz_for_match(16);
    println!("{}", fb_for_match);

    let fb_match_tuple = fizzbuzz_match_tuple(16);
    println!("{}", fb_match_tuple);

    ownership();

    smart_pointer();

    struct_pra();

    enum_pra();

    option_pra();

    crate_pra();

    module_pra();

    lib_pra();

    trait_prac();
}

fn say_hello() {
    println!("Hello!");
    println!("{}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32 {
    // 返したい値をセミコロン無しで返せるらしい
    // returnを使うときは、条件分岐で途中で返すときなど
    // 理由としては、;が無い場合、式としてみなされるため、値が返る。もし;があれば、値が破棄される。
    a + b
}

fn practice_match(x: i32) {
    match x {
        0 => println!("zero"),
        1 => {
            println!("one1");
            println!("one2");
        },
        _ => println!("other"),
    };

    let y:i32 = match x {
        0 => 1,
        1 => 2,
        _ => 0
    };
    println!("{}", y);
}

fn practice_loop() {
    println!("#---loop start---");
    let mut cnt: i32 = 0;
    loop {
        if cnt >= 10 {
            break;
        }
        cnt += 1;
        if cnt == 2 {
            continue;
        }
        println!("{}", cnt);
    }
    println!("#---loop end---");
}

fn practice_while() {
    println!("#---while start---");
    let mut cnt: i32 = 0;
    while cnt <= 5 {
        println!("{}", cnt);
        cnt += 1;
    }
    println!("#---while end---");
}

fn practice_for() {
    for i in [0, 1, 2] {
        println!("Count: {}", i);
    }

    let r = 1..=10;
    for i in r {
        println!("{}", i * i);
    }
}

fn fizzbuzz_while(num: i32) -> i32 {
    let mut cnt: i32 = 1;
    while cnt <= num {
        if cnt % 3 == 0 && cnt % 5 == 0 {
            println!("FizzBuzz");
        } else if cnt % 3 == 0 {
            println!("Fizz");
        } else if cnt % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", cnt);
        }
        cnt += 1;
    }
    num
}

fn fizzbuzz_for_match(num: i32) -> i32 {
    let r = 1..=num;
    for i in r {
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
    num
}

fn fizzbuzz_match_tuple(num: i32) ->i32 {
    let r = 1..=num;
    for i in r {
        match (i%3, i%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
    num
}

fn ownership() {
    // 所有権について
    // 1. 各値は, 各変数（所有者）に対応している
    // 2. 所有者は１つであること（借用や参照はできる）
    // 3. 所有者がスコープから外れると、値も破棄される（所有者と値のライフタイムが同じ）

    let num = 10;
    {
        // ヒープに実際の値が入る
        // スタックにはptr（ヒープの先頭アドレス）とlen（ヒープに格納された値の長さ）、capacity（ヒープに確保したメモリの容量）が格納される
        let mut v1 = vec![0, 1, 2];
        println!("{:?}", v1);

        let v2 = v1;
        println!("{:?}", v2);

        // v1からv2へ所有権が移ったため、エラーになる
        // v1.push(3);
    }// v2の値が破棄されて、メモリが解放される
    println!("#--- copy ---");
    {// コピーについて
        let c1 = vec![1, 2, 3];
        println!("{:?}", c1);
        // 例外として、StringやVectorには、cloneメソッドも存在する
        // 基本的には所有権の移動を優先で使い、cloneを使う際にはサイズが大きくないかなど注意する必要がある
        // それぞれの変数がそれぞれの値を所有することになる
        let c2 = c1.clone();
        println!("{:?}", c1);
        println!("{:?}", c2);
        // 他には数値型、論理型、文字型、タプル（要素が全てCopy型）はcloneを使わなくてもcopyされる
        // コンパイル時にサイズが決定できるため（高速なアクセスが保障される）
        // 厳密には、Copyトレイを実装した型で、移動ではなくcopyされる
    }
    println!("#--- 所有権　関数 ---");
    {// 関数の場合
        // 関数の引数に値を渡す場合と、関数から戻り値を返す場合に所有権が移動する
        let o1: String = String::from("Good");
        let o2: String = String::from("world");

        // 返り値で引数で受けた値も返すことで、引数に渡した後もo1, o2を使えるようにできる
        // でも引数が増えると、返り値も増えるからスマートじゃない
        let (r1, o1, o2) = ownership_fn_multi_return(o1, o2);
        println!("{}", r1);

        // o1とo2の所有権が引数に移動
        let r2: String = ownership_fn(o1, o2);// 返り値がr1に移動
        println!("{}", r2);
        // Error
        // println!("{}", o1);
        // println!("{}", o2);
    }
    println!("#--- copyと移動について ---");
    {// copyと移動について
        let mut v1 = vec![0, 1, 2];
        // println!("{:p}", v1.as_ptr());
        // println!("{:p}", &v1[0]);
        // println!("{}", v1.len());// 3
        // println!("{}", v1.capacity());// 3

        v1.push(4);
        // capacityが3のため、追加すると容量が増え、それに合わせて値がそちらに移動する
        // println!("{:p}", v1.as_ptr());
        // println!("{:p}", &v1[0]);
        // println!("{}", v1.len());// 4
        // println!("{}", v1.capacity());// 6

        // 指しているpointerが同一になる（copyではなく、所有権が移動したということ）
        println!("{:p}", v1.as_ptr());
        let v2 = v1;
        println!("{:p}", v2.as_ptr());

        // copyの場合
        println!("{:p}", v2.as_ptr());
        let v2_clone = v2.clone();
        // copyしているため、v2_cloneの宣言後でも変数を使える
        println!("{:p}", v2.as_ptr());
        println!("{:p}", v2_clone.as_ptr());
    }
    println!("#--- 参照 ---");
    {// 参照
        // 所有権を持たないポインタ
        // - 値を代入しても所有権の移動が起きない
        // - 所有権を持たないため、値のLife timeに影響を与えない
        // 変数に&を付けて生成する
        // 参照した際の型は、通常の型に&が付いた形になる
        // ある値の参照を作ることを借用するという
        // 参照に*をつけることで、実体にアクセス可能
        let x1 = vec![0, 1, 2];
        let x2 = &x1;
        println!("{:p}", x1.as_ptr());
        println!("{:p}", x2.as_ptr());
        // 参照は2種類ある
        // 共有参照
        // - readonlyで変更不可
        // - &を付けて作成する
        // - readonlyなため、同時に複数生成可能（値が書き換わらないため）
        // 可変参照
        // - readと変更が可能
        // - 変数に&mutをつけて作成
        // - ある値の可変参照が存在する場合、その値に参照（共有参照も可変参照も）は作れない（値の読み込み中に書き換わることを防ぐため）

        let o1: String = String::from("Hello");
        let o2: String = String::from("world");
        let r1: String = ownership_reference(&o1, &o2);
        // 参照しているため、所有権が移動しておらずエラーが出ない
        println!("reference_o1:{}", o1);
        println!("reference_o2:{}", o2);
        println!("reference_r1:{}", r1);

    }

    println!("{}", num);
}// numの値が破棄されて、メモリも解放される

fn ownership_fn(s1: String, s2: String) -> String {
    let text = format!("{}, {}", s1, s2);
    text
}

fn ownership_fn_multi_return(s1: String, s2: String) -> (String, String, String) {
    let text: String = format!("{}, {}", s1, s2);
    (text, s1, s2)
}

fn ownership_reference(s1: &String, s2: &String) -> String {
    let text: String = format!("{}, {}", s1, s2);
    text
}


fn lifetime() {
    // 参照が有効になるスコープ
    // - 参照はすべてlifetimeを保持している
    // 基本的にlifetimeも推論される
    // ダングリングポインタ（無効なメモリアドレスを指しているポインタ）を防ぐ目的で使用される
    {
        let r;
        {
            let x: i32 = 10;
            r = &x;
            println!("{}", r);// ここならOK
        }// ここでxのlifetimeが切れるため、期間外で出力するとコンパイルエラーが出る
        // println!("{}", r);
        //つまり、「参照の生存期間は、元の変数の生存期間に完全に含まれていなければならない」
    }

    {
        let mut result: Option<&String> = None;
        let x: String = String::from("123456");
        {
            let y: String = String::from("12345");
            result  = Some(lifetime_longest(&x, &y));
            // ここでresultを使うならうまくいく（yがまだ生きているため安全）
            // if let Some(val) = result {
            //     println!("{}", val);
            // } else {
            //     println!("No result found");
            // }
        }
        // ❌ ここで `result` を使うとコンパイルエラー
        // `result` の `Some(val)` が `&y` を持っている可能性があるが、
        // `y` はすでにスコープを抜けて破棄されている
        // → 借用チェッカーがこれを防ぐ（ダングリングポインタ防止）
        // if let Some(val) = result {
        //     println!("{}", val);
        // } else {
        //     println!("No result found");
        // }
    }
}

fn lifetime_longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    // lifetimeはxとyのどちらか短い方に制約されるという意味
    // 仮にyの方がlifetimeが短ければ、返り値のlifetimeもyと同様になる
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn smart_pointer() {
    // ヒープ領域にi32の値が入り、スタックにポインタが入る。let y = x;などすると、yにポインタの値がわたり、所有権の移動が起き、xは無効になる。
    // ヒープ領域に保存されるため、サイズが未確定でもコンパイルエラーにならない
    let x = Box::new(1);
    println!("x: {:p}", x);
    println!("x_v: {}", x);
    // Box<i32>はポインタなため、*xを使う
    println!("*x +2 = {}", *x + 2);
    let x_stack = 2;
    let x_ = &x_stack;
    println!("x_stack: {:p}", x_);
    println!("x_stack_v: {}", *x_);
    // x_もポインタだが、rustが*x_に自動で読み替えてくれているため、エラーにならない。
    println!("*x +2 = {}", x_ + 2);

    //RCは値が参照されている数をカウントできる。（reference count）
    // RCを使うことで、例外的に複数の所有者を持たせることができる。
    // 参照カウントは、GCで使われる方法。
    // importが必要
    let a = Rc::new("aaa".to_string());
    // aの所有権を渡さないために参照を引数へ渡す
    println!("count: {}", Rc::strong_count(&a));
    {
        let b = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        println!("count &a 2: {}", Rc::strong_count(&a));
        println!("count &b: {}", Rc::strong_count(&b));
    }// bのライフサイクルが終わるため、カウントが減る
    println!("count &a 3: {}", Rc::strong_count(&a));
}

fn struct_pra() {
    // javaやpythonのclassのようなもの
    // 関連のある複数の値をまとめて名前をつけられるデータ型
    // 基本先頭が大文字で、複数の単語で命名するときは、スネークケースで命名する
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // メソッド定義
    // データを持ち、一貫した動作をするもの
    impl Rectangle {
        // methodも所有権の移動が起きるため、参照にする
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // 型関連関数
        // 特定のインスタンスではなく、型そのものに関連づけられた関数のこと（staticメソッドと同様のもの）
        // 型自体に関連付けられているため、インスタンス化することなく使用できる
        // いろんな用途があるが、特にコンストラクタとして使われる
        // 慣習として、newという名前で作る
        // selfを引数に持たなければ、型関連関数になる
        // 戻り値はSelfでも、Rectangleでもok
        fn new(width: u32, height: u32) -> Self {
            Rectangle {width, height}
        }

    }

    {
        let height = 5;
        // インスタンス化
        // mutにすると、インスタンス化した後にも値を書き換えられるようになる
        let mut r = Rectangle{
            width: 10,
            height,
        };
        println!("{}", r.width);
        println!("{}", r.height);
        println!("area: {}", r.area());

        // 値を変更
        r.width = 6;
        println!("{}", r.width);
        println!("area2: {}", r.area());

        // 型関連関数
        let r2 = Rectangle::new(10, 100);
        println!("{}", r2.width);
        println!("{}", r2.height);
        println!("area3: {}", r2.area());
    }
}

fn enum_pra() {
    // 列挙型（取りうる値を列挙する）
    enum Shape {
        Circle,
        Square(u32),// タプル型バリアント
        Triangle{base: u32, height: u32},// 構造体バリアント
    }

    // 構造体と同様にmethodを持つことができる
    // 異なる種類のデータをまとめ、動作を分岐させる場合にenum+implを行う
    impl Shape {
        fn call(&self) {
            println!("called");
        }
    }

    {
        let c = Shape::Circle;
        let s = Shape::Square(2);
        let t = Shape::Triangle {base: 2, height: 3};

        c.call();
        s.call();
        t.call();
    }
}

fn option_pra() {
    // 列挙型で定義された型の一つで、値が存在しない可能性があるものに対して使用する
    // rustにはnullが存在しない
    // option型は以下のように定義されている
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // Optionはよく使われるため、「Option::」を省略できる
    // let a = Option::Some(1);
    let b = Some(2);
    let c = Some("aa");
    // Noneをただ宣言するだけだと、型が未定のため、明示的に型指定をする
    let d: Option<i32> = None;

    let v = vec![0,1,2];
    let value = v.get(1);
    println!("value: {:?}", value);
    // matchかifで存在するか判定しないと使えない
    match value {
        Some(x) => println!("value: {}", x),
        None => println!("none")
    }
    // これでもok
    // ifであれば、全ての分岐は書く必要がない感じ
    // 書いてないもの（none）は無視される
    if let Some(x) = value {
        println!("value: {}", x);
    }

    // matchで存在チェックするとき、特定の値で分岐もできる
    match value {
        // Some(1) => println!("one"),
        // Some(2|3) => println!("two or three"),
        Some(x) if *x == 1 => println!("value: one"),// マッチガード（x<10やx%==0とかができ、より柔軟にマッチできる）
        Some(x) => println!("value: {}", x),
        None => println!("none")
    }
}

fn crate_pra(){
    // クレートについて
    // Rustのプログラムを構成する要素
    // ライブラリや実行ファイルのコード、テストコード、ツール設定などが含まれる
    // クレートには、バイナリクレートとライブラリクレートの２種類がある
    // バイナリクレートは、実行可能なクレート
    // ライブラリクレートは、バイナリクレートから実行ファイルとして呼び出されれて、実行されるクレート

    // パッケージ
    // ある機能群を提供する１つ以上のクレートの集合（パッケージはクレートより大きい概念）
    // cargo newでpackageが作成される
    // cargo.tomlにパッケージの情報を記述する
    // パッケージに含むことができるクレート数は以下の制約がある（両方を含むことも可能）
    // - ライブラリクレート：0 or 1
    // - バイナリクレート：0以上
    // src/main.rsをバイナリクレートと認識し、src/lib.rsをライブラリクレートと認識する
    // cargo newだと、バイナリクレートが一つできる
    // src/binの中にファイルを置くと、それぞれのファイルがバイナリクレートとなる

    // クレートの依存関係
    // クレートから別のクレートも呼び出せる
    // create.ioから他の人が作成したクレートを取得したり、公開することができる
    // create.ioからクレートを使うには、Cargo.tomlにクレート名とバージョンを記載する（cargo runやcargo build時にダウンロードされる）

    // 試しにrandを使ってみる
    let random_num = rand::rng().random_range(1..101);
    println!("random_num: {}", random_num);
}

fn module_pra(){
    // moduleとは
    // クレート内のコードをグループ化して、可読性と再利用性を高める機構
    // - 関数、構造体、列挙型、定数が収められる
    // rustの名前空間として使用される
    // 1ファイル1モジュール（モジュールが肥大化した場合は、複数ファイルにも分割可能）

    // module内の要素の可視性も制御可能
    // - public：要素がmoduleの外からもアクセス可能（pubキーワードをつける）
    // - private：module内 or 子moduleからのみアクセス可能
    // defaultは全てprivate

    // moduleのネストについて
    // ネストされた内側のmoduleをサブモジュールと呼ぶ
    // サブモジュールの要素を外部から使用するには、要素だけではなく、サブモジュール自体もpublicにする必要がある
    // pub(super)とすると、親モジュールからのみアクセス可能になる

    // pathについて
    // モジュールの要素を使用するには、pathの指定が必要
    // pathの種類は以下の２種類
    // - 絶対path：クレートの名前か、crateという文字列から始まる
    // - 相対path：selfやsuper、今のモジュール内の識別子などを使って、現在のモジュールから始まる
    // 区切り文字は::を使う

    // importについて
    // 使いたいmoduleの要素をimportすることで、局所的なaliasを作成でき、pathを省略できるようになる
    // importには、「use」キーワードを使う
    // 要素を直接importせず、その要素を含むmoduleやトレイトをimportして、要素自体には相対pathでアクセスするのが良い
    // ↑一つ上の階層の要素を名前空間として利用し、要素名の衝突を防ぐため

    // 絶対パス
    // 絶対パスはcrateから始まるが、crateはルートクレートのことであり、pathの最上位を表しているため、現在のmain.rsを指すことになる
    crate::test_module::sub_module1::test1();
    // 相対パス
    // selfは自分自身を表すため、今回の場合はmain.rsを指す
    self::test_module::sub_module1::test1();
    // selfは省略もできる
    test_module::sub_module1::test1();

    // test_module2はprivateなため、エラーになる
    // test_module::sub_module2::test1();

    //以下のようにimportすると、短く記載できる
    // use test_module::sub_module1;
    sub_module1::test1();

    let test_struct = sub_module1::TestStruct::new(1, 2);
    test_struct.add();

}

// 試しにつくる
// mod test_module {
//     pub mod sub_module1 {
//         pub fn test1() {
//             println!("test1");
//         }
//         fn test2(){
//             println!("test2");
//         }
//     }
//
//     mod sub_module2 {
//         pub fn test1() {
//             println!("test3");
//         }
//         fn test2(){
//             println!("test4");
//         }
//     }
// }

// 別ファイルのmoduleの読み取り
// セミコロンをつけることで、このmoduleの中身が、test_module.rsファイルにあることがコンパイラに伝わる
mod test_module;

fn lib_pra(){
    rust_lesson::say_hello();
}

fn trait_prac() {
    println!("Trait!");
    let rec = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    let cir = Circle {
        radius: 5.0,
    };

    // traitもuseする必要がある（今回はuse Shape）
    println!("rec_area: {}", rec.calc_area());
    println!("rec_perimeter: {}", rec.calc_perimeter());
    println!("rec_say: {}", )
}