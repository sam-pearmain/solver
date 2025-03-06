#![allow(dead_code)]

use crate::plot3d::read::read_plot3d_ascii;

pub struct Block {
    nx: usize,
    ny: usize,
    nz: usize, 
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
}

impl Block {
    pub fn new() -> Self {
        Block { 
            nx: 0, 
            ny: 0, 
            nz: 0, 
            x: Vec::new(), 
            y: Vec::new(), 
            z: Vec::new(),
        }
    }

    pub fn with_dimensions(mut self, nx: usize, ny: usize, nz: usize) -> Self {
        self.nx = nx;
        self.ny = ny;
        self.nz = nz;
        self
    }

    pub fn set_x_coordinates(&mut self, x: Vec<f64>) -> Result<(), &'static str> {
        if self.nx * self.ny * self.nz == 0 {
            return Err("dimensions not set");
        }
        if x.len() == self.nx {
            self.x = x;
        } else {
            return Err("x data doesn't match dimensions")
        }
        Ok(())
    }

    pub fn set_y_coordinates(&mut self, y: Vec<f64>) -> Result<(), &'static str> {
        if self.nx * self.ny * self.nz == 0 {
            return Err("dimensions not set");
        }
        if y.len() == self.nx {
            self.y = y;
        } else {
            return Err("y data doesn't match dimensions")
        }
        Ok(())
    }

    pub fn set_z_coordinates(&mut self, z: Vec<f64>) -> Result<(), &'static str> {
        if self.nx * self.ny * self.nz == 0 {
            return Err("dimensions not set");
        }
        if z.len() == self.nz {
            self.z = z;
        } else {
            return Err("z data doesn't match dimensions")
        }
        Ok(())
    }

    pub fn from_plot3d(&mut self, p3d_filepath: &str) -> Result<Self, &'static str> {
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
}   
