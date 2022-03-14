use graph::{Dot, RingGraph};

use crate::graph::FullGraph;

mod graph;

fn main() {
    let g = FullGraph::<7>::default();
    println!("{}", Dot(g));
}
