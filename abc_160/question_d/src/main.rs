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
        n: usize,
        x: usize,
        y: usize,
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        graph[i].push(i+1);
        graph[i+1].push(i);
    }
    graph[x-1].push(y-1);
    graph[y-1].push(x-1);
    let mut answer: Vec<i64> = vec![0; n];
    for i in 0..n {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut dist: Vec<i32> = vec![-1; n];
        dist[i] = 0;
        que.push_back(i);
        while !que.is_empty() {
            let p = match que.pop_front() {
                Some(x) => x,
                _ => 0
            };
            let nx_v = dist[p] + 1;
            for k in &graph[p] {
                if dist[*k] != -1 {continue;}
                if i < *k {
                    answer[nx_v as usize] += 1;
                }
                dist[*k] = nx_v;
                que.push_back(*k);
            }
        }
    }

    for i in 1..n {
        println!("{}", answer[i]);
    }
}
