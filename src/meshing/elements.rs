#![allow(dead_code)]

use super::nodes::{Node1D, Node2D};

pub trait Element { 
    const DOF: usize;

    fn volume(&self) -> f64;
}

#[derive(Debug)]
pub struct LineElement<'a> {
    id: usize, 
    n1: &'a Node1D,
    n2: &'a Node1D,
}

pub struct TriangleElement<'a> {
    id: usize, 
    n1: &'a Node2D, //     n3
    n2: &'a Node2D, //   /   \
    n3: &'a Node2D, //  n1---n2
}

impl Element for LineElement<'_> { 
    const DOF: usize = 2; 

    fn volume(&self) -> f64 {
        (self.n1.x - self.n2.x).abs()
    }
}

impl Element for TriangleElement<'_> { 
    const DOF: usize = 3; 

    fn volume(&self) -> f64 {
        0.5 * ((self.n1.x * (self.n2.y - self.n3.y)) +
               (self.n2.x * (self.n3.y - self.n1.y)) +
               (self.n3.x * (self.n1.y - self.n2.y))).abs()
    }
}

impl<'a> LineElement<'a> {
    fn new(id: usize, n1: &'a Node1D, n2: &'a Node1D) -> Self {
        Self { id, n1, n2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meshing::nodes::*;

    #[test]
    fn test_line_element_creation() {
        let n1 = Node1D::new(0, 0.0);
        let n2 = Node1D::new(1, 2.0);
        let line = LineElement::new(0, &n1, &n2);
        println!("{:?}", line);
    }
}