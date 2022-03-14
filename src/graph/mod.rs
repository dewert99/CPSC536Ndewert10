pub use self::{
    full_graph::FullGraph, graph::*, hyper_cube_graph::HyperCubeGraph, ring_graph::RingGraph,
    torus_graph::TorusGraph,
};

mod full_graph;
mod graph;
mod hyper_cube_graph;
mod ring_graph;
mod torus_graph;
