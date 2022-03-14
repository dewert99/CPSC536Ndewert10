use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, Map, Repeat, Take};
use std::ops::{Index, IndexMut, Range};

use rand::Rng;

use crate::graph::{Bin, Graph};

pub struct HyperCubeGraph<const N: usize, const D: usize>(Box<[Bin; N]>);

impl<const N: usize, const D: usize> Default for HyperCubeGraph<N, D> {
    fn default() -> Self {
        HyperCubeGraph(Box::new([0; N]))
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct HyperCubeVertex<const D: usize>(usize);

impl<const D: usize> HyperCubeVertex<D> {
    fn flip(self, rhs: u8) -> HyperCubeVertex<D> {
        HyperCubeVertex(self.0 ^ (1 << rhs))
    }
}

impl<const D: usize> Display for HyperCubeVertex<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0d$b}", self.0, d = D)
    }
}

impl<const N: usize, const D: usize> Index<HyperCubeVertex<D>> for HyperCubeGraph<N, D> {
    type Output = Bin;

    fn index(&self, index: HyperCubeVertex<D>) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<const N: usize, const D: usize> IndexMut<HyperCubeVertex<D>> for HyperCubeGraph<N, D> {
    fn index_mut(&mut self, index: HyperCubeVertex<D>) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

const fn make_equal(a: usize, b: usize) -> usize {
    if a != b {
        panic!()
    }
    a
}

impl<const N: usize, const D: usize> Graph for HyperCubeGraph<N, D> {
    const N: usize = make_equal(N, 1 << D);
    const D: usize = D;
    type Vertex = HyperCubeVertex<D>;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> HyperCubeVertex<D>>;
    type NIter = Map<
        Enumerate<Take<Repeat<HyperCubeVertex<D>>>>,
        fn((usize, HyperCubeVertex<D>)) -> HyperCubeVertex<D>,
    >;

    fn iter_vertices() -> Self::VIter {
        (0..N).map(|x| HyperCubeVertex(x))
    }

    fn iter_neighbours(v: HyperCubeVertex<D>) -> Self::NIter {
        std::iter::repeat(v)
            .take(D)
            .enumerate()
            .map(|(i, v)| v.flip(i as u8))
    }

    fn random_edge(rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = HyperCubeVertex(rng.gen_range(0..N));
        let bit = rng.gen_range(0..D);
        (v, v.flip(bit as u8))
    }
}

#[test]
fn test_valid() {
    HyperCubeGraph::<1, 0>::validate();
    HyperCubeGraph::<2, 1>::validate();
    HyperCubeGraph::<4, 2>::validate();
    HyperCubeGraph::<8, 3>::validate();
    HyperCubeGraph::<32, 5>::validate();
}
