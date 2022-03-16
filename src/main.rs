use graph::{Dot, RingGraph};

use crate::algorithm::{load_balanced, Greedy};
use crate::graph::FullGraph;

mod algorithm;
mod graph;

fn main() {
    let g = load_balanced::<FullGraph<10>, Greedy>(100);
    println!("{}", Dot(g));
}
