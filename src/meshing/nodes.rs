#![allow(dead_code)]

#[derive(Debug)]
pub struct Node1D {
    pub id: usize, 
    pub x:  f64,
}

#[derive(Debug)]
pub struct Node2D {
    pub id: usize, 
    pub x:  f64,
    pub y:  f64,
}

#[derive(Debug)]
pub struct Node3D {
    pub id: usize, 
    pub x:  f64,
    pub y:  f64,
    pub z:  f64,
}

pub trait Node { const DIMENSIONS: usize; }

impl Node for Node1D { const DIMENSIONS: usize = 1; }
impl Node for Node2D { const DIMENSIONS: usize = 2; }
impl Node for Node3D { const DIMENSIONS: usize = 3; }

impl Node1D {
    pub fn new(id: usize, x: f64) -> Self {
        Self { id, x }
    }
}

impl Node2D {
    pub fn new(id: usize, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }
}

impl Node3D {
    pub fn new(id: usize, x: f64, y: f64, z: f64) -> Self {
        Self { id, x, y, z }
    }
}

pub trait Dimensioned {
    fn dimensions() -> usize;
}

impl<T: Node> Dimensioned for T {
    fn dimensions() -> usize {
        Self::DIMENSIONS
    }
}