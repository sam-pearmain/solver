#![allow(dead_code)]

use super::nodes::{Node1D, Node2D};

pub trait Element { const DOF: usize; }

pub struct LineElement<'a> {
    id: usize, 
    n1: &'a Node1D,
    n2: &'a Node1D,
}

pub struct TriangleElement<'a> {
    id: usize, 
    n1: &'a Node2D,
    n2: &'a Node2D,
    n3: &'a Node2D,
}

impl Element for LineElement<'_> { const DOF: usize = 2; }
impl Element for TriangleElement<'_> { const DOF: usize = 3; }