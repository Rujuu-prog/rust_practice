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

