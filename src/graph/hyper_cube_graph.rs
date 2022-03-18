use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, Map, Repeat, Take};
use std::ops::Range;

use rand::Rng;

use crate::graph::Graph;

pub struct HyperCubeGraph {
    pub d: u8,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct HyperCubeVertex(usize);

impl HyperCubeVertex {
    fn flip(self, rhs: u8) -> HyperCubeVertex {
        HyperCubeVertex(self.0 ^ (1 << rhs))
    }
}

impl Display for HyperCubeVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Graph for HyperCubeGraph {
    type Vertex = HyperCubeVertex;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> HyperCubeVertex>;
    type NIter = Map<
        Enumerate<Take<Repeat<HyperCubeVertex>>>,
        fn((usize, HyperCubeVertex)) -> HyperCubeVertex,
    >;

    fn n(&self) -> usize {
        1 << self.d
    }

    fn d(&self) -> usize {
        self.d as usize
    }

    fn as_idx(&self, v: Self::Vertex) -> usize {
        v.0
    }

    fn iter_vertices(&self) -> Self::VIter {
        (0..self.n()).map(|x| HyperCubeVertex(x))
    }

    fn iter_neighbours(&self, v: HyperCubeVertex) -> Self::NIter {
        std::iter::repeat(v)
            .take(self.d())
            .enumerate()
            .map(|(i, v)| v.flip(i as u8))
    }

    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool {
        (v.0 ^ u.0).count_ones() == 1
    }

    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = HyperCubeVertex(rng.gen_range(0..self.n()));
        let bit = rng.gen_range(0..self.d);
        (v, v.flip(bit as u8))
    }
}

#[test]
fn test_valid() {
    HyperCubeGraph { d: 1 }.validate();
    HyperCubeGraph { d: 2 }.validate();
    HyperCubeGraph { d: 3 }.validate();
    HyperCubeGraph { d: 5 }.validate();
}
