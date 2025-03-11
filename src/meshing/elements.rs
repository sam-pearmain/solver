#![allow(dead_code)]

use super::nodes::{Node, Node1D, Node2D, Node3D};

pub trait Element { 
    const DOF: usize;

    fn set_id(&mut self, id: usize);
    fn volume(&self) -> f64;
    fn iter_nodes(&self) -> Box<dyn Iterator<Item = &dyn Node> + '_>;
}

#[derive(Debug)]
pub struct LineElement<'a> {
    id: usize, 
    n1: &'a Node1D,
    n2: &'a Node1D,
}

#[derive(Debug)]
pub struct TriangleElement<'a> {
    id: usize, 
    n1: &'a Node2D, //     n3
    n2: &'a Node2D, //   /   \
    n3: &'a Node2D, //  n1---n2
}

pub struct TetrahedralElement<'a> {
    id: usize, 
    n1: &'a Node3D,
    n2: &'a Node3D,
    n3: &'a Node3D,
    n4: &'a Node3D,
}

impl Element for LineElement<'_> { 
    const DOF: usize = 2; 

    fn volume(&self) -> f64 {
        (self.n1.x - self.n2.x).abs()
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Element for TriangleElement<'_> { 
    const DOF: usize = 3; 

    fn volume(&self) -> f64 {
        0.5 * ((self.n1.x * (self.n2.y - self.n3.y)) +
               (self.n2.x * (self.n3.y - self.n1.y)) +
               (self.n3.x * (self.n1.y - self.n2.y))).abs()
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl<'a> LineElement<'a> {
    fn new(id: usize, n1: &'a Node1D, n2: &'a Node1D) -> Self {
        Self { id, n1, n2 }
    }
}

impl<'a> TriangleElement<'a> {
    fn new(id: usize, n1: &'a Node2D, n2: &'a Node2D, n3: &'a Node2D) -> Self {
        Self { id, n1, n2, n3 }
    }
}

pub struct ElementCollection<T: Element> {
    elements: Vec<T>,
}

impl<T: Element> ElementCollection<T> {
    pub fn new() -> Self {
        ElementCollection { elements: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ElementCollection { elements: Vec::with_capacity(capacity) }
    }

    pub fn push_element(&mut self, e: T) {
        self.elements.push(e);
    }

    pub fn sanitise(&mut self) {
        for (i, element) in self.elements.iter_mut().enumerate() {
            element.set_id(i);
        }
    }

    pub fn get_n_elements(&self) -> usize {
        self.elements.len()
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

    #[test]
    fn test_triangle_element_creation() {
        let n1 = Node2D::new(0, 0.0, 0.0);
        let n2 = Node2D::new(1, 1.0, 0.0);
        let n3 = Node2D::new(2, 1.0, 1.0);
        let tri = TriangleElement::new(0, &n1, &n2, &n3);
        println!("{:?}", tri);
    }
}