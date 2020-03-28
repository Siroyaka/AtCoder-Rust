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
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        r: usize,
        g: usize,
        r_c: usize,
        g_c: usize,
        n_c: usize,
        r_v: [i32; r_c],
        g_v: [i32; g_c],
        n_v: [i32; n_c]
    }
    let mut r_v = r_v;
    r_v.sort();
    let mut g_v = g_v;
    g_v.sort();
    let mut n_v = n_v;
    n_v.sort();
    let mut answer = 0;
    let mut r_count = 0;
    let mut g_count = 0;
    let mut r_ind = 0;
    let mut g_ind = 0;
    let mut n_ind = 0;
    while r_count < r || g_count < g {
        let nitem = if n_ind < n_c {*&n_v[n_c - n_ind - 1]} else {0};
        while r_count < r {
            let p = r_c - r_ind - 1;
            if r_v[p] < nitem {break;}
            answer += r_v[p];
            r_count += 1;
            r_ind += 1;
        }
        while g_count < g {
            let p = g_c - g_ind - 1;
            if g_v[p] < nitem {break;}
            answer += g_v[p];
            g_count += 1;
            g_ind += 1;
        }
        while r_count < r || g_count < g {
            if n_ind == n_c {break;}
            answer+=n_v[n_c - n_ind - 1];
            n_ind += 1;
            if r_count == r && g_count < g {
                g_count += 1;
                continue;
            }
            if g_count == g && r_count < r {
                r_count += 1;
                continue;
            }
            if r_v[r_ind] > g_v[g_ind] {
                r_count += 1;
            } else {
                g_count += 1;
            }
        }
    }

    println!("{}", answer);
}
