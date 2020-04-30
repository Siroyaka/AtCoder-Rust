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
    coin: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.time.cmp(&self.time)
        .then_with(|| self.coin.cmp(&other.coin))
        .then_with(|| self.node.cmp(&other.node))
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
        l: [(usize, usize, usize, i64); m],
        nodes: [(i64, i64); n]
    }
    let max_cost = n * 50;
    let s = cmp::min(max_cost, s as usize);
    let mut graph: Vec<Vec<(usize, usize, i64)>> = vec![vec![]; n];
    for i in 0..m {
        let (f, t, s_cost, m_cost) = l[i];
        graph[f-1].push((t-1, s_cost, m_cost));
        graph[t-1].push((f-1, s_cost, m_cost));
    }

    let mut dist = vec![vec![None; max_cost + 1]; n];
    let mut heep = BinaryHeap::new();
    dist[0][s] = Some(0);
    heep.push(State{time: 0, coin: s, node: 0});

    while let Some(state) = heep.pop() {
        let (node, time, coin) = (state.node, state.time, state.coin);

        // dist[node][coin]にはNoneが入ることはないのでその確認用のassert
        assert!(dist[node][coin].is_some());
        if dist[node][coin] != Some(time) {
            continue;
        }

        // 自分のノードで両替1回した場合の時間とコイン量を保持しておく
        let (ex_c, ex_t) = nodes[node];
        let ex_c = cmp::min(max_cost, ex_c as usize);
        let ex_coin = cmp::min(max_cost, coin + ex_c);
        let ex_time = time + ex_t;
        if (dist[node][ex_coin].is_some() && dist[node][ex_coin] > Some(ex_time)) || !dist[node][ex_coin].is_some() {
            dist[node][ex_coin] = Some(ex_time);
            heep.push(State{time: ex_time, coin: ex_coin, node: node});
        }

        // 行き先のノードの状態を更新する
        for &(next, coin_cost, time_cost) in &graph[node] {
            // そもそも行けない
            if coin < coin_cost {continue;}

            let next_coin = coin - coin_cost;
            let next_time = time + time_cost;

            // 最良ではない
            if dist[next][next_coin].is_some() && dist[next][next_coin] <= Some(next_time) {
                continue;
            }

            dist[next][next_coin] = Some(next_time);
            heep.push(State{time: next_time, coin: next_coin, node: next});
        }
    }
    for i in 1..n {
        let mut min_cost = std::i64::MAX;
        for &p in &dist[i] {
            if let Some(item) = p {
                min_cost = cmp::min(min_cost, item);
            }
        }
        println!("{}", min_cost);
    }
}
