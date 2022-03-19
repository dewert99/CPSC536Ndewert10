use algorithm::{load_balanced, Greedy};
use graph::ArbitraryGraph;

mod algorithm;
mod graph;

fn main() {
    let g = load_balanced::<_, Greedy>(ArbitraryGraph::random(28, 3), 0);
    println!("{}", g);
}
