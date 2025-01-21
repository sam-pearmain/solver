use crate::mesh::Mesh;
use crate::paramters::FluidParameter;

struct SolverSettings {
    iterations: usize,
    time_step: f64,
}

pub struct Solver {
    u_velocity:         Vec<FluidParameter>,
    v_velocity:         Vec<FluidParameter>,
    pressure:           Vec<FluidParameter>,
    density:            Vec<FluidParameter>,
    temperature:        Vec<FluidParameter>,
    solver_settings:    SolverSettings,
    mesh:               Mesh,
}

impl Solver {
    pub fn new() {
        // todo
    }

    pub fn set() {
        // todo
    }
}