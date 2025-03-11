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

pub trait Node {}

impl Node for Node1D {}
impl Node for Node2D {}
impl Node for Node3D {}

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