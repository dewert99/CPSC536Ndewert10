use algorithm::{load_balanced, Greedy};
use graph::RingGraph;

mod algorithm;
mod graph;

fn main() {
    let g = load_balanced::<_, Greedy>(RingGraph { n: 25 }, 500);
    println!("{}", g);
}
