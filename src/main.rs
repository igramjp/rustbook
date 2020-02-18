fn main() {
    println!("compound type");

    // タプル (tuple type)
    // 要素へのアクセス
    // フィールド名を使う 0 始まりの要素番号
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    // フィールド名にはコンパイル時の定数のみ使える．変数は不可
    // let i = 0;
    // let t1a = t1.i;

    // 要素を書き換えるので，変数 t2 に mut を付けて可変にする
    let mut t2 = (88, true);
    // フィールド 0 の要素を書き換える
    t2.0 += 100; // 現在の値に 100 を足す
    assert_eq!(t2, (188, true));

    // パターンマッチで分解する
    let (n1, b1) = (88, true);
    assert_eq!(n1, 88);
    assert_eq!(b1, true);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);

    // 不要な値はアンダースコアを使うと無視できる
    let ((x3, y3), _) = ((0, 5), (10, -1));
    assert_eq!(x3, 0);
    assert_eq!(y3, 5);

    // 要素を書き換えるので，変数 t3 に mut を付けて可変にする
    let mut t3 = ((0, 5), (10, -1));

    // 要素を指す可変の参照を得るために ref mut を追加する
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t3;

    // * を付けることでポインタが指すアドレスにあるデータにアクセスできる
    *x1_ptr += 3;
    *y1_ptr *= -1;

    assert_eq!(t3, ((3, -5), (10, -1)));
}
