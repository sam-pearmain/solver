#![allow(dead_code)]

use std::collections::HashMap;
use crate::plot3d::read::read_plot3d_ascii;
use super::boundary::{Boundary, BoundaryType};

#[derive(Debug)]
pub struct Vertex {
    i: usize, 
    j: usize,
    x: f64,
    y: f64,
}

impl Vertex {
    pub fn new(i: usize, j: usize, x: f64  , y: f64) -> Self {
        Vertex { i, j, x, y }
    }
}

#[derive(Debug)]
pub struct Block {
    nx: usize,
    ny: usize,
    vertices: Vec<Vertex>
}

impl Block {
    pub fn create_with_dimensions(nx: usize, ny: usize) -> Self {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(nx * ny);
        for i in 0..nx {
            for j in 0..ny {
                vertices.push(Vertex::new(i, j, 0.0, 0.0));
            }
        }
        Block { nx, ny, vertices }
    }

    pub fn from_plot3d(p3d_filepath: &str) -> Result<Self, &'static str> {
        let blocks = read_plot3d_ascii(p3d_filepath).expect("cannot read p3d file");
        if blocks.len() > 1 {
            eprintln!("warning: plot3d file contains multiple blocks. only date from the first block will be used");
        }
        let block = blocks.first().expect("cannot read the first block");

        let total_vertices = block.x.len();
        let mut vertices: Vec<Vertex> = Vec::with_capacity(total_vertices);
        let nx = block.imax as usize;
        let ny = block.jmax as usize;

        for j in 0..ny {
            for i in 0..nx {
                let index = i + (j * nx);
                let vertex = Vertex::new(i, j, block.x[index], block.y[index]);
                vertices.push(vertex);
            }
        }

        Ok(Block { nx, ny, vertices })
    }

    pub fn total_grid_points(&self) -> usize {
        self.nx * self.ny
    }

    pub fn export_txt(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;

        writeln!(file, "block dimensions: {} x {}", self.nx, self.ny)?;
        for vertex in &self.vertices {
            writeln!(file, "{} {}, ({:.3}, {:.3})", vertex.i, vertex.j, vertex.x, vertex.y)?;
        }
        
        Ok(())
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block dimensions ({}, {})", self.nx, self.ny)
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