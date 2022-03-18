use std::fmt::{Display, Formatter};
use std::ops::Range;

use rand::Rng;
use smallvec::smallvec;

use crate::graph::Graph;

pub struct RingGraph {
    pub n: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct RingVertex(pub(super) usize);

impl Display for RingVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub(super) const fn calc_d(n: usize) -> usize {
    match n {
        0 => panic!("Graph has no vertices"),
        1 => 0,
        2 => 1,
        _ => 2,
    }
}

impl Graph for RingGraph {
    type Vertex = RingVertex;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> RingVertex>;
    type NIter = smallvec::IntoIter<[RingVertex; 2]>;

    fn n(&self) -> usize {
        self.n
    }

    fn d(&self) -> usize {
        calc_d(self.n)
    }

    fn as_idx(&self, v: Self::Vertex) -> usize {
        v.0
    }

    fn iter_vertices(&self) -> Self::VIter {
        (0..self.n()).map(|x| RingVertex(x))
    }

    fn iter_neighbours(&self, v: RingVertex) -> Self::NIter {
        match self.d() {
            0 => smallvec![],
            1 => smallvec![self.add(v, 1)],
            2 => smallvec![self.add(v, 1), self.add(v, -1)],
            _ => unreachable!(),
        }
        .into_iter()
    }

    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool {
        u == self.add(v, 1) || u == self.add(v, -1)
    }

    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = RingVertex(rng.gen_range(0..self.n()));
        let off = if rng.gen() { 1 } else { -1 };
        (v, self.add(v, off))
    }
}

impl RingGraph {
    fn add(&self, v: RingVertex, rhs: isize) -> RingVertex {
        RingVertex((v.0 as isize + rhs).rem_euclid(self.n as isize) as usize)
    }
}

#[test]
fn test_valid() {
    RingGraph { n: 2 }.validate();
    RingGraph { n: 3 }.validate();
    RingGraph { n: 4 }.validate();
}
