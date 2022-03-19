use std::fmt::{Display, Formatter};

use itertools::Itertools;
use rand::Rng;
use smallvec::{smallvec, SmallVec};

use super::ring_graph::{calc_d, RingVertex};
use super::{Graph, RingGraph};

pub struct TorusGraph {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TorusVertex(RingVertex, RingVertex);

impl Display for TorusVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl Graph for TorusGraph {
    type Vertex = TorusVertex;
    type VIter = std::iter::Map<
        itertools::Product<<RingGraph as Graph>::VIter, <RingGraph as Graph>::VIter>,
        fn((RingVertex, RingVertex)) -> TorusVertex,
    >;
    type NIter = smallvec::IntoIter<[TorusVertex; 4]>;

    fn n(&self) -> usize {
        self.x * self.y
    }

    fn d(&self) -> usize {
        calc_d(self.x) + calc_d(self.y)
    }

    fn as_idx(&self, v: Self::Vertex) -> usize {
        v.0 .0 * self.x + v.1 .0
    }

    fn iter_vertices(&self) -> Self::VIter {
        RingGraph { n: self.x }
            .iter_vertices()
            .cartesian_product(RingGraph { n: self.y }.iter_vertices())
            .map(|(vx, vy)| TorusVertex(vx, vy))
    }

    fn iter_neighbours(&self, v: TorusVertex) -> Self::NIter {
        neighbours(self, v).into_iter()
    }

    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool {
        if v.0 == u.0 {
            RingGraph { n: self.y }.has_edge(v.1, u.1)
        } else if v.1 == u.1 {
            RingGraph { n: self.x }.has_edge(v.0, u.0)
        } else {
            false
        }
    }

    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex) {
        let vx = RingVertex(rng.gen_range(0..self.x));
        let vy = RingVertex(rng.gen_range(0..self.y));
        let v = TorusVertex(vx, vy);
        let neighbours = neighbours(self, v);
        (v, neighbours[rng.gen_range(0..neighbours.len())])
    }
}

fn neighbours(g: &TorusGraph, TorusVertex(vx, vy): TorusVertex) -> SmallVec<[TorusVertex; 4]> {
    let mut res = smallvec![];
    res.extend(
        RingGraph { n: g.x }
            .iter_neighbours(vx)
            .map(|vx| TorusVertex(vx, vy)),
    );
    res.extend(
        RingGraph { n: g.y }
            .iter_neighbours(vy)
            .map(|vy| TorusVertex(vx, vy)),
    );
    res
}

#[test]
fn test_valid() {
    TorusGraph { x: 2, y: 1 }.validate();
    TorusGraph { x: 1, y: 3 }.validate();
    TorusGraph { x: 2, y: 5 }.validate();
    TorusGraph { x: 7, y: 2 }.validate();
    TorusGraph { x: 10, y: 10 }.validate();
}
