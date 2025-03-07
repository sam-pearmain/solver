#![allow(dead_code)]

use crate::grid::{boundary::{Boundary, BoundaryType}, grid::Grid};
use crate::fluid::*;

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
    pub fn new(grid: Grid, fluid: Fluid) -> Self {
        let n_points = grid.get_total_grid_points();
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
        let density = self.fluid.density();
        let gamma = self.fluid.gamma();
        let pressure = 101325.0;
        let kinetic_energy = 0.0;
        let internal_energy = pressure / (gamma - 1.0);
        let total_energy = kinetic_energy + internal_energy;

        for i in 0..self.density.len() {
            self.density[i] = density;
            self.energy[i] = total_energy;
        }
        self.initialise_boundaries()?;
        Ok(())
    }

    fn initialise_boundaries(&mut self) -> Result<(), &'static str> {
        // look inside grid to get each boundary condition
        for (boundary, boundary_type) in self.grid.boundaries.iter() {
            match boundary {
                Boundary::IMax => {}
                Boundary::IMin => {}
                Boundary::JMax => {}
                Boundary::JMin => {}
                Boundary::KMax => {}
                Boundary::KMin => {}
            }
        } 
        
        todo!()
    }
}