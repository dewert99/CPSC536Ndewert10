use rand::Rng;

use crate::graph::{BinGraph, Graph};

use super::LoadBalanceAlgorithm;

pub struct Greedy;

impl<G: Graph> LoadBalanceAlgorithm<G> for Greedy {
    fn for_graph(_: &BinGraph<G>) -> Self {
        Greedy
    }
    fn choose_between(
        &mut self,
        graph: &BinGraph<G>,
        u: G::Vertex,
        v: G::Vertex,
        rng: &mut impl Rng,
    ) -> bool {
        // choose the bin with less balls
        let (u, v) = (graph[u], graph[v]);
        if u < v {
            true
        } else if u == v {
            rng.gen() // break ties with randomness
        } else {
            false
        }
    }
}
