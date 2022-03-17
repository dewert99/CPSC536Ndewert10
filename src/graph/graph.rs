use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::IndexMut;

use itertools::Itertools;
use rand::Rng;

pub type Bin = u32;

pub trait Graph: IndexMut<Self::Vertex, Output = Bin> + Default {
    const N: usize;
    const D: usize;
    const C: () = if Self::D == 0 {
        panic!("Graph has no edges")
    } else {
        ()
    };
    type Vertex: Display + Copy + Hash + Eq;
    type VIter: Iterator<Item = Self::Vertex>;
    type NIter: Iterator<Item = Self::Vertex>;

    fn iter_vertices() -> Self::VIter;
    fn iter_neighbours(v: Self::Vertex) -> Self::NIter;
    fn has_edge(v: Self::Vertex, u: Self::Vertex) -> bool;
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

    fn validate() {
        assert_eq!(Self::iter_vertices().count(), Self::N);
        for v in Self::iter_vertices() {
            assert_eq!(Self::iter_neighbours(v).count(), Self::D);
            for u in Self::iter_neighbours(v) {
                assert!(u != v);
                assert!(Self::iter_neighbours(u).any(|v2| v == v2));
                assert_eq!(Self::iter_neighbours(v).filter(|u2| u == *u2).count(), 1);
                assert!(Self::has_edge(v, u));
                assert!(Self::has_edge(u, v));
            }
        }
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let (u, v) = Self::random_edge(&mut rng);
            assert!(Self::iter_neighbours(u).any(|v2| v == v2), "{u}, {v}");
            assert!(Self::has_edge(u, v));
            assert!(Self::has_edge(v, u));
        }
        Self::C
    }
}

pub struct Dot<G: Graph>(pub G);

impl<G: Graph> Display for Dot<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict graph {{")?;
        writeln!(
            f,
            "label = \"N = {}, D = {}, gap = {}, upper_gap = {}\"",
            G::N,
            G::D,
            self.0.gap(),
            self.0.upper_gap()
        )?;
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
