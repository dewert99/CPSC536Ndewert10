use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, FilterMap, Repeat, Take};
use std::ops::Range;

use rand::Rng;

use crate::graph::Graph;

pub struct FullGraph {
    pub n: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vertex(pub(super) usize);

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Graph for FullGraph {
    type Vertex = Vertex;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> Vertex>;
    type NIter = FilterMap<Enumerate<Take<Repeat<Vertex>>>, fn((usize, Vertex)) -> Option<Vertex>>;

    fn n(&self) -> usize {
        self.n
    }

    fn d(&self) -> usize {
        self.n() - 1
    }

    fn as_idx(&self, v: Self::Vertex) -> usize {
        v.0
    }

    fn iter_vertices(&self) -> Self::VIter {
        (0..self.n()).map(|x| Vertex(x))
    }

    fn iter_neighbours(&self, v: Vertex) -> Self::NIter {
        std::iter::repeat(v)
            .take(self.n())
            .enumerate()
            .filter_map(|(u, v)| if v.0 == u { None } else { Some(Vertex(u)) })
    }

    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool {
        v != u
    }

    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = Vertex(rng.gen_range(0..self.n()));
        let u = Vertex(rng.gen_range(0..self.n()));
        if u == v {
            self.random_edge(rng)
        } else {
            (v, u)
        }
    }
}

#[test]
fn test_valid() {
    FullGraph { n: 2 }.validate();
    FullGraph { n: 3 }.validate();
    FullGraph { n: 10 }.validate();
}
