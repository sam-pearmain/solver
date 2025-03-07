#![allow(dead_code)]

use crate::{grid::grid::Grid, Block};

#[derive(Debug, Clone, Copy)]
pub enum FluidModel {
    IncompressibleEuler,
    CompressibleEuler,
    IncompressibleNavierStokes,
    CompressibleNavierStokes,
}

#[derive(Debug, Clone, Copy)]
pub enum NumericalScheme {
    MacCormack,
}

#[derive(Debug, Clone, Copy)]
pub enum Fluid {
    Air {density: f64, gamma: f64, viscosity: f64},
}

impl Default for Fluid {
    fn default() -> Self {
        Fluid::Air { density: 1.225, gamma: 1.4, viscosity: 1.0 }
    }
}

#[derive(Debug)]
pub struct Solution {
    pub grid: Grid,             // grid of x, y, z coords
    pub fluid: Fluid,           // the simulated fluid and its properties
    pub density: Vec<f64>,      // density (ρ)
    pub momentum_x: Vec<f64>,   // momentum in x-direction (ρu) 
    pub momentum_y: Vec<f64>,   // momentum in y-direction (ρv)
    pub momentum_z: Vec<f64>,   // momentum in z-direction (ρw)
    pub energy: Vec<f64>        // total energy per unit volume (E)
}

impl Solution {
    pub fn new(grid: Block, fluid: Fluid) -> Self {
        let n_points = grid.total_grid_points();
        Solution {
            grid, 
            fluid, 
            density: vec![0.0; n_points],
            momentum_x: vec![0.0; n_points],
            momentum_y: vec![0.0; n_points],
            momentum_z: vec![0.0; n_points],
            energy: vec![0.0; n_points],
        }
    }

    pub fn initialise(&mut self) -> Result<(), &'static str> {
        todo!()
    }
}