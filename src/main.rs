mod graph;
use graph::{Dot, RingGraph};

fn main() {
    let g = RingGraph::<10>::default();
    println!("{}", Dot(g));
}
