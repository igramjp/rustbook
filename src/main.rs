fn main() {
    // ユーザ定義型
    // スタック領域とヒープ 領域

    // 標準ライブラリの主な型
    // Box (std::boxed::Box<T>)
    let t1 = (3, "birds".to_string()); // (i32, String) 型のタプル．スタックに置かれる
    let mut b1 = Box::new(t1); // Box ポインタを作る．タプルがヒープ に移動する
    (*b1).0 += 1; // * で参照外し

    assert_eq!(*b1, (4, "birds".to_string()));

    // ベクタ
    let _v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v2.len(), 4);

    // 長さ100のベクタを作り，全要素を0i32で初期化する
    // (要素の型はCloneトレイトを実装していなければならない)
    let v3 = vec![0; 100]; // Vec<i32>型
    assert_eq!(v3.len(), 100);

    // ベクタは入れ子にできる．子の要素数はそれぞれが異なっても構わない
    let _v4 = vec![vec!['a', 'b', 'c'], vec!['d']]; // Vec<Vec<char>>型

    // ベクタは同じ型の要素の並び．異なる型の要素は持てない
    // let v5 = vec![false, 'a'];

    // ベクタには値を追加・削除する方法がいくつか用意されている
    let mut v6 = vec!['a', 'b', 'c']; // Vec<char>型
    v6.push('d'); // 最後尾に値を追加
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);

    assert_eq!(v6.pop(), Some('e')); // 最後尾から値を取り出し
    v6.insert(1, 'f'); // インデックス1の位置に要素を挿入
    assert_eq!(v6.remove(2), 'b'); // インデックス2の要素を削除．戻り値は削除した値
    assert_eq!(v6, ['a', 'f', 'c', 'd']); // v6の現在の値

    let mut v7 = vec!['g', 'h']; // 別のベクタv7を作成
    v6.append(&mut v7); // v6の最後尾にv7の全要素を追加
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []); // v7は空になった (全要素がv6へ移動した)

    let a8 = ['i', 'j']; // 配列a8を作成
    v6.extend_from_slice(&a8); // 配列a8を作成
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']); // a8は変更なし (a8の要素がコピーされた)

    // 4要素のベクタVec<i32>を作り，要素を1つ足して5要素に拡張する
    let mut v8 = vec![0, 1, 2, 3];
    v8.push(4);
    println!("v8 len: {}, capacity: {}", v8.len(), v8.capacity()); // 5要素だが8要素分のメモリを確保している

    // Box<[i32]>に変換する
    // 余分なメモリを持たなくするためにVecのshrink_to_fit()メソッドが実行されてからBox化される
    let s1 = v8.into_boxed_slice();

    // 余分なメモリを持っていないことを確認するためにVec<i32>に戻す
    let v9 = s1.into_vec();
    println!("v9 len: {}, capacity: {}", v9.len(), v9.capacity()); // 5要素ぴったりのメモリを確保していることがわかる
}
