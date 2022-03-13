use itertools::Itertools;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::ops::IndexMut;
pub type Bin = u32;

pub trait Graph: IndexMut<Self::Vertex, Output = Bin> + Default {
    const N: usize;
    const D: usize;
    type Vertex: Display + Copy;
    type VIter: Iterator<Item = Self::Vertex>;
    type NIter: Iterator<Item = Self::Vertex>;

    fn iter_vertices() -> Self::VIter;
    fn iter_neighbours(v: Self::Vertex) -> Self::NIter;
    fn random_edge(rng: &mut impl Rng) -> (Self::Vertex, Self::Vertex);

    fn gap(&self) -> Bin {
        let minmax = Self::iter_vertices().map(|v| self[v]).minmax();
        let (min, max) = minmax.into_option().unwrap();
        max - min
    }

    fn upper_gap(&self) -> Bin {
        let (sum, max) = Self::iter_vertices()
            .map(|v| self[v])
            .fold((0u64, 0u32), |(sum, max), x| (sum + x as u64, max.max(x)));
        let avg = sum / Self::N as u64;
        max - avg as Bin
    }
}

pub struct Dot<G: Graph>(pub G);

impl<G: Graph> Display for Dot<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict graph {{")?;
        for v in G::iter_vertices() {
            writeln!(f, "  {v} [label = \"{v}:{balls}\"]", balls = self.0[v])?;
            for u in G::iter_neighbours(v) {
                writeln!(f, "  {v} -- {u}")?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
