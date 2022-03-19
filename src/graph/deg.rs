use std::iter::repeat_with;
use std::mem::swap;

use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

type SmallVec = smallvec::SmallVec<[usize; 2]>;

pub(super) struct Pairing {
    d: usize,
    pairs: Box<[[usize; 2]]>,
    //m/2 length list of pairs of points
    edges: Box<[SmallVec]>, // n x n table listing the indexes in 'map' of edges between a cell x cell pair
}

impl Pairing {
    fn n(&self) -> usize {
        self.pairs.len() * 2 / self.d
    }

    fn same_cell(&self, p1: usize, p2: usize) -> bool {
        p1 / self.d == p2 / self.d
    }

    // returns the index into `edges` for edges between p1s and p2s cell
    fn edges_idx(&self, p1: usize, p2: usize) -> usize {
        let c1 = p1 / self.d;
        let c2 = p2 / self.d;
        let (min, max) = if c1 > c2 { (c1, c2) } else { (c2, c1) };
        min * self.n() + max
    }

    // returns the number of edges between p1s and p2s cells
    fn card(&self, p1: usize, p2: usize) -> usize {
        self.edges[self.edges_idx(p1, p2)].len()
    }

    fn replace_pair(&mut self, idx: usize, p1: usize, p2: usize) {
        let [old_p1, old_p2] = self.pairs[idx];
        self.edges[self.edges_idx(old_p1, old_p2)].retain(|x| *x != idx);
        self.pairs[idx] = [p1, p2];
        self.edges[self.edges_idx(p1, p2)].push(idx);
    }

    // returns true if all of the points are in different cells
    fn disjoint<const N: usize>(&self, ps: [usize; N]) -> bool {
        ps.iter().map(|&v| v / self.d).sorted().dedup().count() == N
    }

    // iterates all pairs of cells the list of point pairs (as indexs to 'pairs') that connect them
    fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &SmallVec)> {
        self.edges
            .iter()
            .enumerate()
            .map(|(c, count)| (c / self.n(), c % self.n(), count))
    }

    pub(super) fn cell_pairs(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.pairs
            .iter()
            .map(|&[p1, p2]| (p1 / self.d, p2 / self.d))
    }
}

fn rand_pairing(n: usize, d: usize, rng: &mut impl Rng) -> Pairing {
    assert_eq!((n * d) % 2, 0);
    let mut permutation: Box<[usize]> = (0..n * d).collect();
    permutation.shuffle(rng);
    let map = permutation
        .chunks_exact(2)
        .map(|x| match x {
            &[p1, p2] => [p1, p2],
            _ => unreachable!(),
        })
        .collect();
    rand_pairing_edges(d, map)
}

fn rand_pairing_edges(d: usize, pairs: Box<[[usize; 2]]>) -> Pairing {
    let n = pairs.len() * 2 / d;
    let mut edges: Box<[SmallVec]> = repeat_with(SmallVec::default).take(n.pow(2)).collect();
    let mut res = Pairing {
        d,
        pairs,
        edges: Vec::new().into_boxed_slice(),
    };
    res.pairs.iter().enumerate().for_each(|(idx, &[p1, p2])| {
        edges[res.edges_idx(p1, p2)].push(idx);
    });
    res.edges = edges;
    res
}

// Returns the a list of possible (p1, p6), (p4, p5) points the could be used for l_forward
// represents them as indexes into the list of pairs
fn l_forward_possibilities(l_idx: usize, p: &Pairing) -> Vec<(usize, usize)> {
    let [p2, p3] = p.pairs[l_idx];
    assert!(p.same_cell(p2, p3));
    p.pairs
        .iter()
        .enumerate()
        .flat_map(|(idx1, &[p1, p6])| {
            p.pairs
                .iter()
                .enumerate()
                .filter(move |(_, &[p4, p5])| {
                    p.disjoint([p1, p2, p4, p5, p6])
                        && p.card(p1, p6) == 1
                        && p.card(p4, p5) == 1
                        && p.card(p1, p2) == 0
                        && p.card(p3, p4) == 0
                        && p.card(p5, p6) == 0
                })
                .map(move |(idx2, _)| (idx1, idx2))
        })
        .collect()
}

fn no_loops(p: &mut Pairing, loop_idxs: Vec<usize>, rng: &mut impl Rng) -> Option<()> {
    for loop_idx in loop_idxs {
        let [p2, p3] = p.pairs[loop_idx];
        let l_forward_possibilities = l_forward_possibilities(loop_idx, p);
        let &(idx1, idx2) = l_forward_possibilities.choose(rng).unwrap();
        let [p1, p6] = p.pairs[idx1];
        let [p4, p5] = p.pairs[idx2];
        // TODO randomly fail sometimes
        p.replace_pair(loop_idx, p1, p2);
        p.replace_pair(idx1, p3, p4);
        p.replace_pair(idx2, p5, p6);
    }
    Some(())
}

// Returns the a list of possible (p1, p5), (p4, p8) points the could be used for d_forward
// represents them as indexes into the list of pairs
fn d_forward_possibilities(d_idx1: usize, d_idx2: usize, p: &Pairing) -> Vec<(usize, usize)> {
    let [p2, p6] = p.pairs[d_idx1];
    let [p3, p7] = p.pairs[d_idx2];
    assert!(p.same_cell(p2, p3));
    assert!(p.same_cell(p6, p7));

    p.pairs
        .iter()
        .enumerate()
        .flat_map(|(idx1, &[p1, p5])| {
            p.pairs
                .iter()
                .enumerate()
                .filter(move |(_, &[p4, p8])| {
                    p.disjoint([p1, p2, p4, p5, p6, p8])
                        && p.card(p1, p5) == 1
                        && p.card(p4, p8) == 1
                        && p.card(p1, p2) == 0
                        && p.card(p3, p4) == 0
                        && p.card(p5, p6) == 0
                        && p.card(p7, p8) == 0
                })
                .map(move |(idx2, _)| (idx1, idx2))
        })
        .collect()
}

fn no_doubles(p: &mut Pairing, double_idxs: Vec<(usize, usize)>, rng: &mut impl Rng) -> Option<()> {
    for (d_idx1, d_idx2) in double_idxs {
        let [p2, p6] = p.pairs[d_idx1];
        let [p3, p7] = &mut p.pairs[d_idx2];
        if p2 / p.d == *p7 / p.d {
            swap(p3, p7) // reorder to force p2--p3, and p6--p7
        }
        let [p3, p7] = [*p3, *p7];
        let d_forward_possibilities = d_forward_possibilities(d_idx1, d_idx2, p);
        let &(idx1, idx2) = d_forward_possibilities.choose(rng).unwrap();
        let [p1, p5] = p.pairs[idx1];
        let [p4, p8] = p.pairs[idx2];
        // TODO randomly fail sometimes
        p.replace_pair(d_idx1, p1, p2);
        p.replace_pair(d_idx2, p3, p4);
        p.replace_pair(idx1, p5, p6);
        p.replace_pair(idx2, p7, p8);
    }
    Some(())
}

// Core of the deg algorithm, return None to indicate we need to retry
fn try_deg(n: usize, d: usize, rng: &mut impl Rng) -> Option<Pairing> {
    let mut pairing = rand_pairing(n, d, rng);
    let m = n * d;
    let m2 = n * d * (d - 1);
    let mut double_pairs = vec![];
    let mut loops = vec![];
    for (c1, c2, edge_group) in pairing.enumerate() {
        let card = edge_group.len();
        if c1 == c2 {
            //loop
            match card {
                0 => (),
                1 => loops.push(edge_group[0]),
                _ => return None,
            }
        } else {
            match card {
                0 | 1 => (),
                2 => double_pairs.push((edge_group[0], edge_group[1])),
                _ => return None,
            }
        }
    }
    if double_pairs.len() > (m2 / m).pow(2) {
        return None;
    }
    if loops.len() > m2 / m {
        return None;
    }
    no_loops(&mut pairing, loops, rng)?;
    no_doubles(&mut pairing, double_pairs, rng)?;
    Some(pairing)
}

// Algorithm based on https://doi.org/10.1016/0196-6774(90)90029-E
pub(super) fn deg(n: usize, d: usize, rng: &mut impl Rng) -> Pairing {
    loop {
        match try_deg(n, d, rng) {
            None => {}
            Some(p) => return p,
        }
    }
}
