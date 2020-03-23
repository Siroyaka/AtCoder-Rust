

pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}
 
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }
 
        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        // 配列の末尾から降順になっていない箇所を検索する。降順になっている箇所をsuffixとする。
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // iはsuffixの最も左を位置する。
        // suffixの最も左が配列の冒頭だった場合は辞書順の最大要素になっているため、次の要素を作成できない。
        if i == 0 {
            return false;
        }
 
        // suffixの左の位置は降順になっていない値の位置(pivot) 例: [0, 3, 2, 1]の0の位置
        let pivot = i - 1;

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        // pivotより大きい値でかつsuffixの中で最も右にある値(実質最も小さい値)を探す。
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[pivot] {
            j -= 1;
        }
 
        // Step 3: Swap that element with the pivot
        // pivotとpivotより大きくsuffixの中で最も右にある要素を入れ替える。(この時点で入れ替えた後の最大要素が作成される)
        self.swap(j, pivot);
 
        // Step 4: Reverse the (previously) weakly decreasing part
        // 入れ替え終わったsuffixをひっくり返す。(suffixが最小の配置になるようにする)
        self[i..].reverse();
 
        true
    }
 
    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }
 
        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
 
        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }
 
        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();
 
        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
 
        // Step 4: Swap that element with the pivot
        self.swap(i - 1, j);
 
        true
    }
}

fn main() {
    let mut p = (0..10).collect::<Vec<usize>>();
    p[2..].reverse();

    for i in p {
        println!("{:?}", i);
    }
}
