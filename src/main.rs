fn main() {
    // ユーザ定義型
    // スタック領域とヒープ 領域

    // 標準ライブラリの主な型
    // Box (std::boxed::Box<T>)
    let t1 = (3, "birds".to_string()); // (i32, String) 型のタプル．スタックに置かれる
    let mut b1 = Box::new(t1);         // Box ポインタを作る．タプルがヒープ に移動する
    (*b1).0 += 1;                      // * で参照外し

    assert_eq!(*b1, (4, "birds".to_string()));
}
