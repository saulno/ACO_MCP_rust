use super::graph::Graph;

pub struct Parameters {
    pub graph: Graph,
    pub cycles: usize,
    pub ants: usize,
    pub alpha: f32,
    pub rho: f32,
    pub tau_max: f32,
    pub tau_min: f32
}