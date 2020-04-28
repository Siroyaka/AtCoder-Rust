#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    time: i64,
    coin: i64,
    node: usize,
    costnode: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.time.cmp(&self.time)
        .then_with(|| self.coin.cmp(&other.coin))
        .then_with(|| self.node.cmp(&other.node))
        .then_with(|| self.costnode.cmp(&other.costnode))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input!{
        n: usize,
        m: usize,
        s: i64,
        l: [(usize, usize, i64, i64); m],
        nodes: [(i64, i64); n]
    }
    let mut graph: Vec<Vec<(usize, i64, i64)>> = vec![vec![]; n];
    for i in 0..m {
        let (f, t, s_cost, m_cost) = l[i];
        graph[f-1].push((t-1, s_cost, m_cost));
        graph[t-1].push((f-1, s_cost, m_cost));
    }

    let mut dist: Vec<Vec<i64>> = vec![vec![std::i64::MAX; 2500]; n];
    let mut alreadys: Vec<Vec<bool>> = vec![vec![false; 2500]; n];
    for i in 1..2500 {
        alreadys[0][i] = true;
    }
    let mut heep = BinaryHeap::new();
    heep.push(State{time: 0, coin: s, node: 0, costnode: 0});

    while let Some(state) = heep.pop() {
        if alreadys[state.node][state.costnode] {continue;}
        alreadys[state.node][state.costnode] = true;
        
        for &(n_node, s_cost, m_cost) in &graph[state.node] {
            for m_cost in 0..n_node*50 {
                if alreadys[n_node][m_cost] {continue;}
                let n_time;
                let n_coin;
                let nx_cost = s_cost + m_cost as i64;
                if state.coin < nx_cost {
                    let (c_coin, c_time) = nodes[state.node];
                    let times = (nx_cost - state.coin) / c_coin 
                        + if (nx_cost - state.coin) % c_coin == 0 {0} else {1};
                    
                }
                dist[n_node][m_cost] = cmp::min(dist[n_node][m_cost], n_time);
            }
        }
    }

}
