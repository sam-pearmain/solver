#![allow(dead_code)]

pub struct Node1D {
    id: usize, 
    x:  f64,
}

pub struct Node2D {
    id: usize, 
    x:  f64,
    y:  f64,
}

pub struct Node3D {
    id: usize, 
    x:  f64,
    y:  f64,
    z:  f64,
}

pub trait Node { const DIMENSIONS: usize; }

impl Node for Node1D { const DIMENSIONS: usize = 1; }
impl Node for Node2D { const DIMENSIONS: usize = 2; }
impl Node for Node3D { const DIMENSIONS: usize = 3; }

impl Node1D {
    fn new(id: usize, x: f64) -> Self {
        Self { id, x }
    }
}

impl Node2D {
    fn new(id: usize, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }
}

impl Node3D {
    fn new(id: usize, x: f64, y: f64, z: f64) -> Self {
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