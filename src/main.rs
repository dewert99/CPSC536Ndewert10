use graph::{Dot, RingGraph};

use crate::graph::TorusGraph;

mod graph;

fn main() {
    let g = TorusGraph::<1, 3>::default();
    println!("{}", Dot(g));
}
