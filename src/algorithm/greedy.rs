use rand::Rng;

use crate::graph::Graph;

use super::LoadBalanceAlgorithm;

pub struct Greedy;

impl<G: Graph> LoadBalanceAlgorithm<G> for Greedy {
    fn for_graph(_: &G) -> Self {
        Greedy
    }
    fn choose_between(
        &mut self,
        graph: &G,
        u: G::Vertex,
        v: G::Vertex,
        rng: &mut impl Rng,
    ) -> bool {
        let (u, v) = (graph[u], graph[v]);
        if u < v {
            true
        } else if u == v {
            rng.gen()
        } else {
            false
        }
    }
}
