use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Range};

use rand::Rng;
use tinyvec::{array_vec, ArrayVecIterator};

use crate::graph::{Bin, Graph};

pub struct RingGraph<const N: usize>(Box<[Bin; N]>);

impl<const N: usize> Default for RingGraph<N> {
    fn default() -> Self {
        RingGraph(Box::new([0; N]))
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct RingVertex<const N: usize>(pub(super) usize);

impl<const N: usize> Add<isize> for RingVertex<N> {
    type Output = RingVertex<N>;

    fn add(self, rhs: isize) -> Self::Output {
        RingVertex((self.0 as isize + rhs).rem_euclid(N as isize) as usize)
    }
}

impl<const N: usize> Display for RingVertex<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const N: usize> Index<RingVertex<N>> for RingGraph<N> {
    type Output = Bin;

    fn index(&self, index: RingVertex<N>) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<const N: usize> IndexMut<RingVertex<N>> for RingGraph<N> {
    fn index_mut(&mut self, index: RingVertex<N>) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

pub(super) const fn calc_d(n: usize) -> usize {
    match n {
        1 => 0,
        2 => 1,
        _ => 2,
    }
}

impl<const N: usize> Graph for RingGraph<N> {
    const N: usize = N;
    const D: usize = calc_d(N);
    type Vertex = RingVertex<N>;
    type VIter = std::iter::Map<Range<usize>, fn(usize) -> RingVertex<N>>;
    type NIter = ArrayVecIterator<[RingVertex<N>; 2]>;

    fn iter_vertices() -> Self::VIter {
        (0..N).map(|x| RingVertex(x))
    }

    fn iter_neighbours(v: RingVertex<N>) -> Self::NIter {
        match Self::D {
            0 => array_vec![],
            1 => array_vec![v + 1],
            2 => array_vec![v + 1, v + -1],
            _ => unreachable!(),
        }
        .into_iter()
    }

    fn random_edge(rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let v = RingVertex(rng.gen_range(0..N));
        let off = if rng.gen() { 1 } else { -1 };
        (v, v + off)
    }
}

#[test]
fn test_valid() {
    RingGraph::<1>::validate();
    RingGraph::<2>::validate();
    RingGraph::<3>::validate();
    RingGraph::<10>::validate();
}
