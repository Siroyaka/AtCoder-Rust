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

fn factorial(a: usize) -> usize {
    let mut m = a;
    let mut ans = 1;
    while m > 1 {
        ans *= m;
        m -= 1;
    };
    ans
}

fn multi_choose(size: usize, max_value: usize) -> Vec<Vec<usize>> {
    let len = factorial(size + max_value - 1) / factorial(size) / factorial(max_value - 1);
    let mut ans = Vec::new();
    ans.push(vec![max_value; size]);
    let mut hold = vec![max_value; size];
    for _ in 0..len-1 {
        let mut v = 0;
        loop {
            hold[v] -= 1;
            if hold[v] == 0 {
                hold[v] = max_value;
                v += 1;
                continue;
            }
            break;
        }
        for i in 1..size {
            hold[size - i - 1] = cmp::min(hold[size - i - 1], hold[size - i]);
        }
        ans.push(hold.to_vec());
    }
    ans
}

fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        v: [(usize1, usize1, usize, i64); q]
    }

    let t = multi_choose(n, m);
    let mut ans = 0;
    for i in t {
        let mut l = 0;
        for &(a, b, c, d) in &v {
            if i[b] - i[a] == c { l += d; }
        }
        ans = cmp::max(ans, l);
    }
    println!("{}", ans);
}
