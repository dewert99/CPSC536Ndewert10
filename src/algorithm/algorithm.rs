use rand::Rng;

use crate::graph::{BinGraph, Graph};

pub trait LoadBalanceAlgorithm<G: Graph> {
    fn for_graph(graph: &BinGraph<G>) -> Self;
    fn choose_between(
        &mut self,
        graph: &BinGraph<G>,
        u: G::Vertex,
        v: G::Vertex,
        rng: &mut impl Rng,
    ) -> bool;
}

pub fn load_balance<G: Graph, A: LoadBalanceAlgorithm<G>>(graph: &mut BinGraph<G>, balls: u64) {
    let mut algorithm = A::for_graph(graph);
    let mut rng = rand::thread_rng();
    for _ in 0..balls {
        let (u, v) = graph.random_edge(&mut rng);
        if algorithm.choose_between(graph, u, v, &mut rng) {
            graph[u] += 1
        } else {
            graph[v] += 1
        }
    }
}

pub fn load_balanced<G: Graph, A: LoadBalanceAlgorithm<G>>(g: G, balls: u64) -> BinGraph<G> {
    let mut graph = BinGraph::new(g);
    load_balance::<G, A>(&mut graph, balls);
    graph
}
