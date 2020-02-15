fn main() {
    // スカラ型
    // ユニット (unit type)
    // 関数を呼び出し (ないはずの) 戻り値に変数 ret を束縛する
    let ret = hello();
    // アサーションで ret の値がユニット値と等しいことを検査する
    assert_eq!(ret, ());
    // size_of::<型>() は，その型の値がメモリ上で占める大きさをバイト数で表す
    assert_eq!(std::mem::size_of::<()>(), 0); // 0 バイト

    // 真理値 (bool type)
    let b1 = true;
    let _b2 = !b1;

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 >= 10; // false
    let b4 = n2 >= 10; // true
    let _b5 = b3 && b4; // false ショートサーキット論理積
    let _b6 = b3 || b4; // true ショートサーキット論理和

    assert_eq!(std::mem::size_of::<bool>(), 1); // サイズは 1 バイト

    // 整数リテラル
    let _n3 = 10_000; // i32 型 (整数リテラルのデフォルトの型)
    let _n4 = 0u8; // u8 型 サフィックスで型を指定)
    let n5 = -100_isize; // isize 型 (同上)

    // 型推論が働く例
    let n6 = 10; // n6 は isize 型になる．なぜなら，
    let _n7 = n5 + n6; // ここで isize 型の n5 に加算しているから

    // プレフィックスとして 0x, 0o, 0b を付けると順に 16 進数, 8 進数, 2 進数として解釈される
    let _h1 = 0xff; // i32 型, 16 進数
    let _o1 = 0o744; // i32 型, 8 進数
    let _b1 = 0b1010_0110_1110_1001; // i32 型, 2 進数

    // 以下の様に書くと ASCII 文字に対応する文字コードが得られる
    // 型はデフォルトで u8
    let n8 = b'A'; // ASCII 文字 'A' の文字コード 65u8 を得る
    assert_eq!(n8, 65u8);

    // 整数演算桁あふれ
    let _n9 = std::u8::MAX; // u8 型の最大値は 255u8
    let _n10 = 1u8;

    // 答えは 256 だが u8 型では表現できない (オーバーフロー)
    // let n11 = n9 + n10;
    // println!("std::u8::MAX = {}", n9);
    // println!("{}", n11);

    let n12 = 200u8;
    let n13 = 3u8;

    // n12 * n13 = 600 を計算する
    // std::u8::MAX は 255 なので桁あふれする

    // 検査付き乗算 -> None になる
    assert_eq!(n12.checked_mul(n13), None);

    // 飽和乗算 -> u8 の最大値 255 に張り付く
    assert_eq!(n12.saturating_mul(n13), std::u8::MAX);

    // ラッピング乗算 -> 600 を 256 で割った余りの 88 になる
    assert_eq!(n12.wrapping_mul(n13), 88);

    // 桁あふれ乗算 -> 88 と桁あふれを示す true のペアを返す
    assert_eq!(n12.overflowing_mul(n13), (88, true));

    // 固定精度の浮動小数点数
    let _f1 = 10.0; // f64 型 (小数リテラルのデフォルトの型)
    let _f2 = -1_234.56f32; // f32 型 (サフィックスで型を指定)
    let f3 = 578.6E+7; // f64 型 (指数部も指定できる)
    println!("578.6E+7 = {}", f3);

    // RPN
    /*
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);
    debug_assert_eq!("26.2840", format!("{:.4}", ans));
    println!("{} = {:.4}", exp, ans);
    */
}

// スカラ型
// ユニット
fn hello() {
    println!("hello");
}

// RPN
/*
fn rpn(exp: &str) -> f64 {
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}
*/
