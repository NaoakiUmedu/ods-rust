use std::collections::{HashSet, VecDeque};

use interfaces::USet;
mod interfaces;

/// 入力を1行づつ読み、その逆順で出力せよ。
fn t1_1_1(input: Vec<&str>) -> Vec<&str> {
    let mut rev = input.clone();
    rev.reverse();
    rev
}

/// 先頭から50行の入力を読み、それを逆順で出力せよ。
/// その後、続く50行を読み、それを逆順で出力せよ。
/// これを読み取る行がなくなるまで繰り返し、
/// 最後に残った行（50行未満かもしれない）もやはり逆順で出力せよ。
fn t1_1_2(input: Vec<String>) -> Vec<String> {
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
    } else {
        result.append(&mut buf.clone());
    }
    result
}

/// 入力を1行ずつ読み取り、42行め以降で空行を見つけたら、その42行前の行を出力せよ。
/// 例えば、242行めが空行であれば200行めを出力せよ。
/// なお、プログラムの実行中に43行以上の行を保持してはならない。
fn t1_1_3(input: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut buff: VecDeque<String> = VecDeque::new();
    for now in input {
        if buff.len() >= 42 {
            let previous42 = buff.pop_front();
            if now.is_empty() {
                match previous42 {
                    Some(s) => result.push(s),
                    None => (),
                }
            }
        }
        buff.push_back(now);
    }
    result
}

/// 入力を1行ずつ読み取り、それまでに読み込んだことがある行と重複しない行を見つけたら出力せよ。
/// 重複が多いファイルを読む場合でも、重複なく行を保持するのに必要なメモリより多くのメモリを使わないように注意せよ。
fn t1_1_4(input: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut existed: HashSet<String> = HashSet::new();
    for now in input {
        if !existed.contains(&now) {
            result.push(now.clone());
        }
        existed.insert(now.clone());
    }
    result
}

/// 入力を1行ずつ読み取り、それまでに読み込んだことがある行と同じなら出力せよ
/// （最終的には、ある行が入力ファイルに初めて現れた箇所をそれぞれ除いたものが出力になる）。
/// 重複が多いファイルを読む場合でも、重複なく行を保持するのに必要なメモリより多くのメモリを使わないように注意せよ。
fn t_1_1_5(input: Vec<String>) -> Vec<String> {
    let mut first_contacted: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();
    for now in input {
        let mut is_contacted = false;
        for contacted in &first_contacted {
            if now == *contacted {
                result.push(now.clone());
                is_contacted = true;
            }
        }

        if !is_contacted {
            first_contacted.push(now.clone());
        }
    }
    result
}

/// 入力をすべて読み取り、短い順に並べ替えて出力せよ。
/// 同じ長さの行があるときは、それらの行は辞書順に並べるものとする。
/// また、重複する行は一度だけ出力するものとする。
fn t_1_1_6(input: Vec<String>) -> Vec<String> {
    let result: HashSet<String> = HashSet::from_iter(input.iter().cloned());
    let mut result_vec: Vec<String> = result.iter().cloned().collect();
    result_vec.sort();
    result_vec
}

/// 直前の問題で、重複する行については現れた回数だけ出力するように変更せよ
fn t_1_1_7(input: Vec<String>) -> Vec<String> {
    let mut result = input;
    result.sort();
    result
}

/// 入力をすべて読み取り、すべての偶数番めの行を出力したあとに、すべての奇数番めの行を出力せよ
/// （最初の行を0 行めと数える）。
fn t_1_1_8(input: Vec<String>) -> Vec<String> {
    let mut even_result: Vec<String> = Vec::new();
    let mut odd_result: Vec<String> = Vec::new();
    for i in 0..input.len() {
        if i % 2 == 0 {
            even_result.push(input[i].clone());
        } else {
            odd_result.push(input[i].clone());
        }
    }
    even_result.append(&mut odd_result);
    even_result
}

/// 入力をすべて読み取り、ランダムに並べ替えて出力せよ。
/// どの行の内容も書き換えてはならない。
/// また、入力より行が増えたり減ったりしてもいけない。
fn t_1_1_9(input: Vec<String>) -> Vec<String> {
    let mut outputted: Vec<bool> = vec![false; input.len()];
    let mut result: Vec<String> = Vec::new();
    loop {
        let rnd = rand::random::<usize>() % input.len();
        if outputted[rnd] {
            continue;
        } else {
            result.push(input[rnd].clone());
            outputted[rnd] = true;
        }

        if result.len() == input.len() {
            break;
        }
    }
    result
}

// T1.2
// +1がstackのpush/-1がstackのpopに対応し、
// Dyck wordがstackのサイズが0以上になるpushとpopの操作の並びとなる

// T1.3
// 左の括弧が出てきたら1つpush、右の括弧がでてきたらpopして、
// 最後にスタックの要素が0こだったらOK

// T1.4
// sの中身を全部qにpopしたあとqの中身を全部sにpushすれば逆になるだけということ？？？

// T1.5
/// Bag構造体
struct Bag<T: PartialEq> {
    elements: Vec<T>,
}
impl<T: PartialEq> USet<T> for Bag<T> {
    /// サイズ
    fn size(&self) -> usize {
        self.elements.len()
    }
    /// 値xを追加する
    fn add(&mut self, x: T) -> bool {
        self.elements.push(x);
        // 重複する要素も許すため常にtrue
        true
    }
    /// 値xを削除する
    fn remove(&mut self, x: T) -> Option<T> {
        // removeはどうすんだ？とりあえず1こだけ消すようにする
        for i in 0..self.elements.len() {
            if x.eq(&self.elements[i]) {
                self.elements.remove(i);
                return Some(x);
            }
        }
        Option::from(None)
    }
    /// 値xを検索する
    fn find(&self, x: T) -> Option<T> {
        for i in 0..self.elements.len() {
            if x.eq(&self.elements[i]) {
                return Some(x);
            }
        }
        Option::from(None)
    }
}
impl<T: PartialEq> Bag<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    pub fn find_all(self) -> Vec<T> {
        self.elements
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter1::{self, Bag};

    use super::interfaces::USet;

    #[test]
    fn t1_1_1_1() {
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

        assert_eq!(expect, chapter1::t1_1_1(input));
    }

    #[test]
    fn t1_1_2_1() {
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

        assert_eq!(expect, chapter1::t1_1_2(input));
    }

    #[test]
    fn t1_1_3_1() {
        let mut input: Vec<String> = (0..242).map(|x| i32::to_string(&x)).collect();
        input[41] = String::from("");
        input[141] = String::from("");
        input[241] = String::from("");
        let expect: Vec<String> = vec!["99".to_string(), "199".to_string()];

        assert_eq!(expect, chapter1::t1_1_3(input));
    }

    #[test]
    fn t1_1_4_1() {
        let input: Vec<String> = vec!["aaa", "iii", "iii", "uuu", "iii", "ooo", "eee", "kkk"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect: Vec<String> = vec!["aaa", "iii", "uuu", "ooo", "eee", "kkk"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(expect, chapter1::t1_1_4(input));
    }

    #[test]
    fn t1_1_5_1() {
        let input: Vec<String> = vec!["aaa", "iii", "uuu", "iii", "iii", "aaa", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect: Vec<String> = vec!["iii", "iii", "aaa", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(chapter1::t_1_1_5(input), expect);
    }

    #[test]
    fn t_1_1_6_1() {
        let input: Vec<String> = vec!["aaa", "iii", "uuu", "iii", "iii", "aaa", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect: Vec<String> = vec!["aaa", "iii", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(chapter1::t_1_1_6(input), expect);
    }

    #[test]
    fn t_1_1_7_1() {
        let input: Vec<String> = vec!["aaa", "iii", "uuu", "iii", "iii", "aaa", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect: Vec<String> = vec!["aaa", "aaa", "iii", "iii", "iii", "uuu", "uuu"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(chapter1::t_1_1_7(input), expect);
    }

    #[test]
    fn t_1_1_8_1() {
        let input: Vec<String> = vec!["aaa", "iii", "uuu", "eee", "ooo"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect: Vec<String> = vec!["aaa", "uuu", "ooo", "iii", "eee"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(chapter1::t_1_1_8(input), expect);
    }

    #[test]
    fn t_1_1_9_1() {
        let input: Vec<String> = vec!["aaa", "iii", "uuu", "eee", "ooo"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expect = input.len();

        assert_eq!(chapter1::t_1_1_9(input).len(), expect);
    }

    #[test]
    fn t_1_5() {
        let mut bag: Bag<i32> = Bag::new();
        bag.add(1);
        bag.add(1);
        bag.add(2);
        bag.add(3);
        bag.add(2);
        bag.remove(1);
        assert_eq!(4, bag.size());

        let finded = bag.find(2);
        assert_eq!(Some(2), finded);
        assert_eq!(None, bag.find(4));

        let mut tmp = bag.find_all();
        tmp.sort();
        let expected = vec![1, 2, 2, 3];
        for i in 0..tmp.len() {
            assert_eq!(expected[i], tmp[i]);
        }
    }
}
