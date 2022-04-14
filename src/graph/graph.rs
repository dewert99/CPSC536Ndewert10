use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::iter::repeat;
use std::ops::{Deref, Index, IndexMut};

use itertools::Itertools;
use rand::Rng;

pub type Bin = u32;

pub trait Graph {
    type Vertex: Display + Copy + Hash + Eq;
    type VIter: Iterator<Item = Self::Vertex>;
    type NIter: Iterator<Item = Self::Vertex>;

    fn n(&self) -> usize;
    fn d(&self) -> usize;
    fn m(&self) -> usize {
        let m2 = self.n() * self.d();
        debug_assert!(m2 % 2 == 0);
        m2 / 2
    }

    fn as_idx(&self, v: Self::Vertex) -> usize;

    fn iter_vertices(&self) -> Self::VIter;
    fn iter_neighbours(&self, v: Self::Vertex) -> Self::NIter;
    fn has_edge(&self, v: Self::Vertex, u: Self::Vertex) -> bool;
    fn random_edge(&self, rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex);

    fn validate(&self) {
        assert_eq!(self.iter_vertices().count(), self.n());
        for v in self.iter_vertices() {
            assert_eq!(self.iter_neighbours(v).count(), self.d());
            for u in self.iter_neighbours(v) {
                assert!(u != v);
                assert!(self.iter_neighbours(u).any(|v2| v == v2));
                assert_eq!(self.iter_neighbours(v).filter(|u2| u == *u2).count(), 1);
                assert!(self.has_edge(v, u));
                assert!(self.has_edge(u, v));
            }
        }
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let (u, v) = self.random_edge(&mut rng);
            assert!(self.iter_neighbours(u).any(|v2| v == v2), "{u}, {v}");
            assert!(self.has_edge(u, v));
            assert!(self.has_edge(v, u));
        }
    }
}

// Wraps a graph by attaching bins to each of it's vertices
pub struct BinGraph<G: Graph> {
    graph: G,
    data: Box<[Bin]>,
}

impl<G: Graph> Deref for BinGraph<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<G: Graph> Index<G::Vertex> for BinGraph<G> {
    type Output = Bin;

    fn index(&self, index: G::Vertex) -> &Self::Output {
        &self.data[self.as_idx(index)]
    }
}

impl<G: Graph> IndexMut<G::Vertex> for BinGraph<G> {
    fn index_mut(&mut self, index: G::Vertex) -> &mut Self::Output {
        &mut self.data[self.as_idx(index)]
    }
}

impl<G: Graph> BinGraph<G> {
    pub fn new(graph: G) -> Self {
        let n = graph.n();
        BinGraph {
            graph,
            data: repeat(0).take(n).collect(),
        }
    }

    pub fn gap(&self) -> Bin {
        let minmax = self.iter_vertices().map(|v| self[v]).minmax();
        let (min, max) = minmax.into_option().unwrap();
        max - min
    }

    pub fn upper_gap(&self) -> Bin {
        let (sum, max) = self
            .iter_vertices()
            .map(|v| self[v])
            .fold((0u64, 0u32), |(sum, max), x| (sum + x as u64, max.max(x)));
        let avg = sum / self.n() as u64;
        max - avg as Bin
    }
}

impl<G: Graph> Display for BinGraph<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict graph {{")?;
        writeln!(
            f,
            "label = \"n = {}, d = {}, gap = {}, upper_gap = {}\"",
            self.n(),
            self.d(),
            self.gap(),
            self.upper_gap()
        )?;
        for v in self.iter_vertices() {
            writeln!(f, "  {v} [label = \"{v}:{balls}\"]", balls = self[v])?;
            for u in self.iter_neighbours(v) {
                writeln!(f, "  {v} -- {u}")?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
