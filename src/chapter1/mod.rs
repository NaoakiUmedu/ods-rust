/// 入力を1行づつ読み、その逆順で出力せよ。
fn t1(input: Vec<&str>) -> Vec<&str> {
    let mut rev = input.clone();
    rev.reverse();
    rev
}

/// 先頭から50行の入力を読み、それを逆順で出力せよ。
/// その後、続く50行を読み、それを逆順で出力せよ。
/// これを読み取る行がなくなるまで繰り返し、
/// 最後に残った行（50行未満かもしれない）もやはり逆順で出力せよ。
fn t2(input: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut buf: Vec<String> = Vec::new();
    let mut cnt = 0;
    for i in 0..input.len() {
        buf.push(input[i].clone());
        cnt = cnt + 1;
        if cnt == 0 {
            // DO Nothing!
        } else if cnt % 100 == 0 {
            result.append(&mut buf.clone());
            buf = Vec::new();
            cnt = 0;
        } else if cnt % 50 == 0 {
            buf.reverse();
            result.append(&mut buf.clone());
            buf = Vec::new();
        }
    }
    if cnt <= 50 {
        buf.reverse();
        result.append(&mut buf.clone());
        buf = Vec::new();
    } else {
        result.append(&mut buf.clone());
        buf = Vec::new();
        cnt = 0;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::chapter1;

    #[test]
    fn t1_1() {
        let input = vec![
            "春過ぎて",
            "夏来にけらし",
            "白妙の",
            "衣ほすてふ",
            "天の香具山",
        ];

        let expect = vec![
            "天の香具山",
            "衣ほすてふ",
            "白妙の",
            "夏来にけらし",
            "春過ぎて",
        ];

        assert_eq!(expect, chapter1::t1(input));
    }

    #[test]
    fn t2_1() {
        let input: Vec<String> = (0..120).map(|x| i32::to_string(&x)).collect();
        let expect_num: Vec<i32> = vec![
            49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28,
            27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5,
            4, 3, 2, 1, 0, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 119, 118, 117, 116, 115, 114, 113, 112, 111,
            110, 109, 108, 107, 106, 105, 104, 103, 102, 101, 100,
        ];
        let expect: Vec<String> = expect_num.iter().map(|x| i32::to_string(&x)).collect();

        assert_eq!(expect, chapter1::t2(input));
    }
}
