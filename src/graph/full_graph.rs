use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, FilterMap, Repeat, Take};
use std::ops::{Index, IndexMut, Range};

use rand::Rng;

use crate::graph::{Bin, Graph};

pub struct FullGraph<const N: usize>(Box<[Bin; N]>);

impl<const N: usize> Default for FullGraph<N> {
    fn default() -> Self {
        FullGraph(Box::new([0; N]))
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vertex<const N: usize>(pub(super) usize);

impl<const N: usize> Display for Vertex<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const N: usize> Index<Vertex<N>> for FullGraph<N> {
    type Output = Bin;

    fn index(&self, index: Vertex<N>) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<const N: usize> IndexMut<Vertex<N>> for FullGraph<N> {
    fn index_mut(&mut self, index: Vertex<N>) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl<const N: usize> Graph for FullGraph<N> {
    const N: usize = N;
    const D: usize = N - 1;
    type Vertex = Vertex<N>;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> Vertex<N>>;
    type NIter =
        FilterMap<Enumerate<Take<Repeat<Vertex<N>>>>, fn((usize, Vertex<N>)) -> Option<Vertex<N>>>;

    fn iter_vertices() -> Self::VIter {
        (0..N).map(|x| Vertex(x))
    }

    fn iter_neighbours(v: Vertex<N>) -> Self::NIter {
        std::iter::repeat(v)
            .take(N)
            .enumerate()
            .filter_map(|(u, v)| if v.0 == u { None } else { Some(Vertex(u)) })
    }

    fn has_edge(v: Self::Vertex, u: Self::Vertex) -> bool {
        v != u
    }

    fn random_edge(rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = Vertex(rng.gen_range(0..N));
        let u = Vertex(rng.gen_range(0..N));
        if u == v {
            Self::random_edge(rng)
        } else {
            (v, u)
        }
    }
}

#[test]
fn test_valid() {
    FullGraph::<2>::validate();
    FullGraph::<3>::validate();
    FullGraph::<10>::validate();
}
