#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
 
    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
 
    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };
 
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
 
    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };
 
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
 
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("parse error")
    };
}

fn main() {
    input!{
        n: usize,
        s: chars
    }
    let s = s.into_iter().rev().collect::<Vec<_>>();
    let mut answer: i64 = 0;
    for i in 0..n {
        let own = s[i];
        let mut count_a = 0;
        let mut a = 's';
        let mut count_b = 0;
        let mut queue_a: VecDeque<usize> = VecDeque::new();
        let mut queue_b: VecDeque<usize> = VecDeque::new();
        for j in i+1..n {
            let i_a = if queue_a.len() == 0 {0} else {queue_a[0]};
            let i_b = if queue_b.len() == 0 {0} else {queue_b[0]};
            if s[j] != own && a == 's' {
                a = s[j];
                count_a += 1;
                queue_a.push_back(j + (j - i));
                continue;
            }

            let mut mul_a = count_a;
            let mut mul_b = count_b;
            if i_a == j {
                queue_a.pop_front();
                mul_a = cmp::max(0, mul_a - 1);
            }
            if i_b == j {
                queue_b.pop_front();
                mul_b = cmp::max(0, mul_b - 1);
            }
            if s[j] == own {continue;}

            if s[j] == a {
                count_a += 1;
                queue_a.push_back(j + (j - i));
                answer += mul_b;
                continue;
            }
            count_b += 1;
            queue_b.push_back(j + (j - i));
            answer += mul_a;
        }
    }
    println!("{}", answer);
}
