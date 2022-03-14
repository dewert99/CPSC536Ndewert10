use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use itertools::Itertools;
use rand::Rng;
use tinyvec::{array_vec, ArrayVecIterator};

use crate::graph::ring_graph::calc_d;
use crate::graph::{Bin, Graph, RingVertex};
use crate::RingGraph;

pub struct TorusGraph<const X: usize, const Y: usize>(Box<[[Bin; Y]; X]>);

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct TorusVertex<const X: usize, const Y: usize>(RingVertex<X>, RingVertex<Y>);

impl<const X: usize, const Y: usize> Display for TorusVertex<X, Y> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl<const X: usize, const Y: usize> Default for TorusGraph<X, Y> {
    fn default() -> Self {
        TorusGraph(Box::new([[0; Y]; X]))
    }
}

impl<const X: usize, const Y: usize> Index<TorusVertex<X, Y>> for TorusGraph<X, Y> {
    type Output = Bin;

    fn index(&self, index: TorusVertex<X, Y>) -> &Self::Output {
        &self.0[index.0 .0][index.1 .0]
    }
}

impl<const X: usize, const Y: usize> IndexMut<TorusVertex<X, Y>> for TorusGraph<X, Y> {
    fn index_mut(&mut self, index: TorusVertex<X, Y>) -> &mut Self::Output {
        &mut self.0[index.0 .0][index.1 .0]
    }
}

impl<const X: usize, const Y: usize> Graph for TorusGraph<X, Y> {
    const N: usize = X * Y;
    const D: usize = calc_d(X) + calc_d(Y);
    type Vertex = TorusVertex<X, Y>;
    type VIter = std::iter::Map<
        itertools::Product<<RingGraph<X> as Graph>::VIter, <RingGraph<Y> as Graph>::VIter>,
        fn((RingVertex<X>, RingVertex<Y>)) -> TorusVertex<X, Y>,
    >;
    type NIter = ArrayVecIterator<[TorusVertex<X, Y>; 4]>;

    fn iter_vertices() -> Self::VIter {
        RingGraph::<X>::iter_vertices()
            .cartesian_product(RingGraph::<Y>::iter_vertices())
            .map(|(vx, vy)| TorusVertex(vx, vy))
    }

    fn iter_neighbours(TorusVertex(vx, vy): TorusVertex<X, Y>) -> Self::NIter {
        let mut res = array_vec![];
        res.extend(RingGraph::iter_neighbours(vx).map(|vx| TorusVertex(vx, vy)));
        res.extend(RingGraph::iter_neighbours(vy).map(|vy| TorusVertex(vx, vy)));
        res.into_iter()
    }

    fn random_edge(rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let vx = RingVertex(rng.gen_range(0..Self::N));
        let vy = RingVertex(rng.gen_range(0..Self::N));
        let offx = if rng.gen() { 1 } else { -1 };
        let offy = if rng.gen() { 1 } else { -1 };
        (TorusVertex(vx, vy), TorusVertex(vx + offx, vy + offy))
    }
}

#[test]
fn test_valid() {
    TorusGraph::<1, 1>::validate();
    TorusGraph::<2, 1>::validate();
    TorusGraph::<1, 3>::validate();
    TorusGraph::<2, 5>::validate();
    TorusGraph::<7, 2>::validate();
    TorusGraph::<10, 10>::validate();
}
