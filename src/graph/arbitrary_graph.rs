use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::vec::IntoIter;

use rand::Rng;

use crate::graph::Graph;

pub struct ArbitraryGraph {
    d: usize,
    data: Box<[Vertex]>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vertex(usize);

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Graph for ArbitraryGraph {
    type Vertex = Vertex;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> Vertex>;
    type NIter = IntoIter<Vertex>;

    fn n(&self) -> usize {
        self.data.len() / self.d
    }

    fn d(&self) -> usize {
        self.d
    }

    fn as_idx(&self, v: Self::Vertex) -> usize {
        v.0
    }

    fn iter_vertices(&self) -> Self::VIter {
        (0..self.n()).map(|x| Vertex(x))
    }

    fn iter_neighbours(&self, v: Vertex) -> Self::NIter {
        let res = self.neighbours(v).to_vec();
        res.into_iter()
    }

    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool {
        self.neighbours(v).contains(&u)
    }

    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = Vertex(rng.gen_range(0..self.n()));
        let off = rng.gen_range(0..self.d());
        (v, self.neighbours(v)[off])
    }
}

impl ArbitraryGraph {
    pub fn random(n: usize, d: usize) -> ArbitraryGraph {
        let pairing = super::deg::deg(n, d, &mut rand::thread_rng());
        let mut res = vec![vec![]; n];
        pairing.cell_pairs().for_each(|(v1, v2)| {
            res[v1].push(v2);
            res[v2].push(v1);
        });
        let res = res
            .into_iter()
            .flat_map(|v| {
                assert_eq!(v.len(), d);
                v.into_iter().map(|v| Vertex(v))
            })
            .collect();
        ArbitraryGraph { data: res, d }
    }

    fn neighbours(&self, v: Vertex) -> &[Vertex] {
        &self.data[v.0 * self.d..(v.0 * self.d + self.d)]
    }
}

#[test]
fn test_valid() {
    for _ in 1..10 {
        ArbitraryGraph::random(28, 3).validate()
    }
}
