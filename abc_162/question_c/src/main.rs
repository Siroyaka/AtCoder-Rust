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

fn uc(a : i64, b : i64) -> i64{
    fn ucc(la : i64, sm : i64) -> i64 {
        let ans = la % sm;
        if ans == 0 {
            return sm;
        }
        ucc(sm, ans)
    }
    if a > b {ucc(a, b)} else {ucc(b, a)}
}

fn main() {
    input!{
        n: usize
    }
    let mut answer = 0;
    for i in 1..n+1 {
        let ii = i as i64;
        for j in 1..n+1 {
            let jj = j as i64;
            let uc1 = uc(ii, jj);
            for k in 1..n+1 {
                let kk = k as i64;
                answer += uc(uc1, kk);
            }
        }
    }
    println!("{}", answer);
}
