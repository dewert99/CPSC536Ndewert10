use rand::Rng;

use crate::graph::{BinGraph, Graph};

// Generic interface for algorithms that solve the graphical balls and bins problem
pub trait LoadBalanceAlgorithm<G: Graph> {
    fn for_graph(graph: &BinGraph<G>) -> Self;
    // Returns true to indicate throwing a ball into bin 'u'
    fn choose_between(
        &mut self,
        graph: &BinGraph<G>,
        u: G::Vertex,
        v: G::Vertex,
        rng: &mut impl Rng,
    ) -> bool;
}

// Throws 'balls' additional balls into an existing graphical balls and bins setup
pub fn load_balance<G: Graph, A: LoadBalanceAlgorithm<G>, R: Rng>(
    graph: &mut BinGraph<G>,
    balls: u64,
    rng: &mut R,
) {
    let mut algorithm = A::for_graph(graph);
    for _ in 0..balls {
        let (u, v) = graph.random_edge(rng);
        if algorithm.choose_between(graph, u, v, rng) {
            graph[u] += 1
        } else {
            graph[v] += 1
        }
    }
}

// Runs the graphical balls and bins process with 'balls' balls on the graph 'g'
pub fn load_balanced<G: Graph, A: LoadBalanceAlgorithm<G>>(g: G, balls: u64) -> BinGraph<G> {
    let mut graph = BinGraph::new(g);
    let mut rng = rand::thread_rng();
    load_balance::<G, A, _>(&mut graph, balls, &mut rng);
    graph
}
