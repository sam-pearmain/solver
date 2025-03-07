#![allow(dead_code)]

use crate::grid::{boundary::{Boundary, BoundaryType}, grid::Grid};

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
        let (density, gamma, _) = match self.fluid {
            Fluid::Air { density, gamma, viscosity } => (density, gamma, viscosity)
        };
        let velocity_x: f64 = 0.0;
        let velocity_y: f64 = 0.0;
        let velocity_z: f64 = 0.0;
        let pressure = 101325.0;
        let kinetic_energy = 0.5 * density * (
            velocity_x.powi(2) +
            velocity_y.powi(2) +
            velocity_z.powi(2)
        );
        let internal_energy = pressure / (gamma - 1.0);
        let total_energy = kinetic_energy + internal_energy;

        for i in 0..self.density.len() {
            self.density[i] = density;
            self.momentum_x[i] = density * velocity_x;
            self.momentum_y[i] = density * velocity_y;
            self.momentum_z[i] = density * velocity_z;
            self.energy[i] = total_energy;
        }
        self.initialise_boundaries()?;
        Ok(())
    }

    fn initialise_boundaries(&mut self) -> Result<(), &'static str> {
        // look inside grid to get each boundary condition
        for boundary_condition in self.grid.boundaries.iter() {
            self.apply_boundary(boundary_condition);
        } 
        
        todo!()
    }

    fn apply_boundary(&mut self, boundary_condition: (&Boundary, &mut BoundaryType)) -> Result<(), &'static str> {

    }
}