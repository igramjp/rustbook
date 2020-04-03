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

    // 配列 (array type)
    let _a1 = [false, true, false]; // [bol; 3] 型
    let a2 = [0.0, -1.0, 1.0, 0.5]; // [f64; 4] 型

    // len() で配列の長さを得られる
    assert_eq!(a2.len(), 4);

    // 長さ 100 の配列を作り，全要素を 0i32 で初期化する
    // (要素の型は Copy トレイトを実装していなければならない)
    let a3 = [0; 100]; // [i32; 100] 型
    assert_eq!(a3.len(), 100);

    // 配列は入れ子にできる
    let _a4 = [['a', 'b'], ['c', 'd']]; // [[char; 2]; 2] 型．2 次元配列

    // 配列は同じ型の要素の並び．異なる型の要素は持てない
    // let a5 = [false, 'a']; // error

    // 配列の長さは実行時に指定できない
    let size = 100;
    // let _a5 = [0; size]; // error コンパイル時定数が要求される場所に定数でない値がある

    // ベクタなら実行時に長さを指定できる
    let mut v1 = vec![0; size]; // Vec<i32> 型
    assert_eq!(v1.len(), 100);

    // ベクタには要素を追加したり，削除したりできる．
    v1.push(1); // ベクタの最後尾に要素を追加する
    assert_eq!(v1.len(), 101); // 長さは 100
    assert_eq!(v1.pop(), Some(1)); // ベクタの最後尾から要素を取り除く
    assert_eq!(v1.len(), 100); // 長さは 100 に戻る

    // 要素へのアクセス
    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくても構わない
    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    let array3 = [0, 1];
    // array3[2]; // error インデックスが範囲外

    // let index = 2;
    // array3[index]; // コンパイルエラーにならず，実行時にパニックする

    // get() メソッドを使うとパニックしない
    assert_eq!(array3.get(1), Some(&1)); // get() はインデックスが範囲内のときは Some(&値) を返す
    assert_eq!(array3.get(2), None); // さもなければ None を返す

    // イテレータを使う
    let array4 = ['a'; 50]; // 長さ 50

    // iter() で要素が不変のイテレータを作成
    for ch in array4.iter() {
        print!("{}", *ch);
    }
    println!();

    let mut array5 = [1; 32];

    // iter_mut() で要素が可変のイテレータを作成
    for n in array5.iter_mut() {
        *n *= 2;
    }
    println!("{:?}", array5);

    // スライス (slice type)
    // 配列
    let a6 = ['a', 'b', 'c', 'd']; // 参照元のデータ [char; 4] 型
    println!("a6: {:?}", a6);

    print_info("&a6[..]", &a6[..]); // &[char] 型．全要素のスライス
    print_info("&a6", &a6); // 同上
    print_info("&a6[1..3]", &a6[1..3]); // b と c を要素とする長さ 2 のスライス

    // ベクタ
    let v1 = vec!['e', 'f', 'g', 'h'];
    println!("v1: {:?}", v1);

    print_info("&v1[..]", &v1[..]); // &[char] 型．全要素のスライス
    print_info("&v1", &v1); // 同上
    print_info("&v1[1..3]", &v1[1..3]); // &[char]型．f と g を要素とする長さ 2 のスライス

    // ミュータブルなスライス
    let mut a7 = [5, 4, 3, 2]; // 配列．[i32; 4] 型
    let s1 = &mut a7[1..3]; // 可変スライス．&mut[i32] 型
    s1[0] = 6; // スライスの最初の要素を 6 にする
    s1[1] *= 10; // 2 番目の要素を 10 倍する
    s1.swap(0, 1); // 要素を交換する
    assert_eq!(s1, [30, 6]); // スライスの内容を確認

    // 参照元の配列の内容を確認
    assert_eq!(a7, [5, 30, 6, 2]); // スライスを通じて配列の内容が変更された

    // スライスに対する主な操作
    let a8: [i32; 0] = [];
    let s2 = &a8; // 不変のスライスを作成
    assert!(s2.is_empty()); // 空のスライス
    assert_eq!(s2.len(), 0); // 長さは 0
    assert_eq!(s2.first(), None); // 最初の要素は存在しない

    let a9 = ["zero", "one", "two", "three", "four"];
    let s3 = &a9[1..4]; // 不変のスライスを作成
    assert!(!s3.is_empty()); // 空ではない
    assert_eq!(s3.len(), 3); // 長さは 3
    assert_eq!(s3.first(), Some(&"one")); // 最初の要素

    assert_eq!(s3[1], "two"); // 2 番目の要素
                              // assert_eq!(s3[3], "?"); // 4 番目の要素．存在しないのでパニックする
    assert_eq!(s3.get(1), Some(&"two")); // 2 番目の要素を得る別の方法
    assert_eq!(s3.get(3), None); // 4 番目の要素．存在しないので None

    assert!(s3.contains(&"two")); // "two" を要素に持つ
    assert!(s3.starts_with(&["one", "two"])); // "one", "two" で始まる
    assert!(s3.ends_with(&["two", "three"])); // "two", "three" で終わる

    // 次は可変のスライスだけで可能な操作
    let mut a10 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    // 一部の要素を昇順にソートする
    // &mut を省略しても結果は同じ．型強制によって自動的にスライスが作られる
    &mut a10[2..6].sort();
    assert_eq!(&a10[2..6], &[0, 2, 8, 9]);

    // スライスを 2 つの可変スライスへ分割する
    let (s4a, s4b) = &mut a10.split_at_mut(5);

    // 前半を逆順にする
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);

    // 後半を昇順にソートする
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // sort() と sort_unstable() の違い
    // sort() は安定ソートなので同順なデータのソート前の順序がソート後も保存される
    // sort_unstable() は安定ソートではないが，一般的に sort() より高速

    // 文字列スライス
    let str1 = "abc1"; // &'static str 型
    let str2 = "abc2";
    assert!(str1 < str2);
    assert!(str1 != str2);

    let str3 = "文字列を複数行に渡って書くと
        改行やスペースが入る";
    let str4 = "行末にバックスラッシュを付けると\
        改行などが入らない";
    assert_eq!(
        str3,
        "文字列を複数行に渡って書くと\n        改行やスペースが入る"
    );
    assert_eq!(str4, "行末にバックスラッシュを付けると改行などが入らない");

    let str5 = "文字列に\"と\\を含める"; // バックスラッシュでエスケープ
    let str6 = r#"文字列に"と\を含める"#; // raw 文字列リテラル．正規表現などに便利
    assert_eq!(str5, str6);

    let str7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    assert_eq!(str7, "このように#の数を増やすと\"##\"があっても大丈夫");

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";

    // lines() メソッドは改行コード(\n)を含む文字列から1行ずつ
    // 取り出せるイテレータを作る
    let mut lines = fruits.lines();
    // イテレータの next() メソッドで次の行を得る
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    assert_eq!(lines.next(), None);

    // りんごの行 (Some(..)) の中身を取り出す
    if let Some(apples) = apple_line {
        // 「あか」で始まるかチェック
        assert!(apples.starts_with("あか"));
        // 「りんご」の文字を含むかチェック
        assert!(apples.contains("りんご"));
        // 「あお」が最初に出現する位置 (UTF-8表現で何バイト目) を得る
        assert_eq!(apples.find("あお"), Some(17)); // 0 始まりなので 18 バイト目

        // 文字列をカンマ (,) で分割するイテレータを作る
        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));

        let green = apple_iter.next();
        // 左側に余白がある
        assert_eq!(green, Some(" あおりんご"));
        // Some(..) の内容に str の trim() メソッドを適用して余白を取り除く
        assert_eq!(green.map(str::trim), Some("あおりんご"));

        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!();
    }

    // s5 から s8 はどれも画面上では 1 文字として表示される
    let s5 = "a"; // 61
    let s6 = "あ"; // E3 81 *2
    let s7 = "😄"; // F0 9F 98 80
    let s8 = "🇯🇵"; // F0 9F 87 AF F0 9F 87 B5

    // len() メソッドは UTF-8 のバイト数を返す
    assert_eq!(s5.len(), 1);
    assert_eq!(s6.len(), 3);
    assert_eq!(s7.len(), 4);
    assert_eq!(s8.len(), 8);

    let s9 = "abcあいう";
    assert_eq!(s9.get(0..1), Some("a"));
    assert_eq!(s9.get(3..6), Some("あ"));
    assert_eq!(s9.get(3..4), None); // UTF-8 として解釈できない場合

    let s10 = "かか\u{3099}く";
    println!("{}", s10);

    let mut iter_s10 = s10.chars(); // chars() の代わりに char_indices() を使うと char と str 上の開始バイトがペアになったタプルが得られる
    assert_eq!(iter_s10.next(), Some('か'));
    assert_eq!(iter_s10.next(), Some('か'));
    assert_eq!(iter_s10.next(), Some('\u{3099}'));
    assert_eq!(iter_s10.next(), Some('く'));
    assert_eq!(iter_s10.next(), None);

    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));

    let bad_utf8: [u8; 2] = [0x81, 0x33]; // でたらめなバイト列
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("{:?}", result2);

    // 文字列リテラル (&'static str) から &mut str は直接得られない
    // まず文字列リテラルを String へ変換し，そこから &mut str を取り出す
    let mut string1 = "abcあいう".to_string(); // String 型

    // &mut str を得る．これは String が持つ UTF-8 バイト列を指す可変スライス
    let str8 = string1.as_mut_str(); // &mut str 型

    // 英小文字を大文字に変更
    str8.make_ascii_uppercase();
    assert_eq!(str8, "ABCあいう");

    // &mut str の UTF-8 バイト列を直接操作して "あ" ( 3 バイト) を "*a*" に変更する
    let b = unsafe { str8.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';

    // 大元の String が変更されている
    assert_eq!(string1, "ABC*a*いう");
}

// この関数は &[char] 型のスライスを引数に取り，その情報を表示する
fn print_info(name: &str, sl: &[char]) {
    println!(
        "  {:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),   // 長さ (バイト数) usize 型
        sl.first(), // 最初の要素      Option<char> 型
        sl[1],      // 2 番目の要素    char 型
        sl.last()   // 最後の要素      Option<char> 型
    );
}
