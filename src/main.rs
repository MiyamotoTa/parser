/// 位置情報。 .0から.1までの区間を表す
/// たとえばLoc(4, 6) なら入力文字の5文字目から7文字目までの区間を表す(0始まり)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// Locに便利メソッドを実装しておく
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

fn main() {
    println!("Hello, world!");
}
