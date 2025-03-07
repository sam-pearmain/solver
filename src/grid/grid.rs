#![allow(dead_code)]

use std::collections::HashMap;
use crate::plot3d::read::read_plot3d_ascii;
use super::boundary::{Boundary, BoundaryType};

#[derive(Debug)]
pub struct Vertex {
    i: usize, 
    j: usize, 
    k: usize, 
    x: f64,
    y: f64,
    z: f64,
}

impl Vertex {
    pub fn new(
        i: usize, j: usize, k: usize,
        x: f64  , y: f64,   z: f64,
    ) -> Self {
        Vertex { i, j, k, x, y, z }
    }
}

#[derive(Debug)]
pub struct Block {
    nx: usize,
    ny: usize,
    nz: usize,
    vertices: Vec<Vertex>
}

impl Block {
    fn new() -> Self {
        Block { 
            nx: 0, 
            ny: 0, 
            nz: 0, 
            vertices: Vec::new(),        
        }
    }

    pub fn with_dimensions(mut self, nx: usize, ny: usize, nz: usize) -> Self {
        self.nx = nx;
        self.ny = ny;
        self.nz = nz;
        self
    }

    pub fn initialise_vertices()

    pub fn from_plot3d(p3d_filepath: &str) -> Result<Self, &'static str> {
        let blocks = read_plot3d_ascii(p3d_filepath)
            .expect("cannot read p3d file");
        if blocks.len() > 1 {
            return Err("only single block meshes are currently supported");
        }
        let p3d_block = blocks.first().unwrap();
        let mut block = Block::new()
            .with_dimensions(
                p3d_block.imax as usize, 
                p3d_block.jmax as usize, 
                p3d_block.kmax as usize, 
            );
        block.set_x_coordinates(p3d_block.x.clone())?;
        block.set_y_coordinates(p3d_block.y.clone())?;
        block.set_z_coordinates(p3d_block.z.clone())?;
        Ok(block)
    }

    pub fn total_grid_points(&self) -> usize {
        self.nx * self.ny * self.nz
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block dimensions ({}, {}, {})", self.nx, self.ny, self.nz)
    }
}

#[derive(Debug)]
pub struct Grid {
    block: Block,
    pub boundaries: HashMap<Boundary, BoundaryType>,
}

impl Grid {
    pub fn get_total_grid_points(&self) -> usize {
        self.block.total_grid_points()
    }
}