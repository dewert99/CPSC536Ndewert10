use graph::{Dot, RingGraph};

use crate::graph::{HyperCubeGraph, TorusGraph};

mod graph;

fn main() {
    let g = TorusGraph::<3, 10>::default();
    println!("{}", Dot(g));
}
