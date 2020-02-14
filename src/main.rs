fn main() {
    // スカラ型
    // ユニット (unit type)
    // 関数を呼び出し (ないはずの) 戻り値に変数 ret を束縛する
    let ret = hello();
    // アサーションで ret の値がユニット値と等しいことを検査する
    assert_eq!(ret, ());
    // size_of::<型>() は，その型の値がメモリ上で占める大きさをバイト数で表す
    assert_eq!(std::mem::size_of::<()>(), 0); // 0 バイト

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
