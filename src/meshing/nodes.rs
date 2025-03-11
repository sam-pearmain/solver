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

pub trait Node { 
    const DIMENSIONS: usize;

    fn set_id(&mut self, id: usize); 
}

impl Node for Node1D { 
    const DIMENSIONS: usize = 1;

    fn set_id(&mut self, id: usize) {
        self.id = id
    } 
}

impl Node for Node2D { 
    const DIMENSIONS: usize = 2; 

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Node for Node3D { 
    const DIMENSIONS: usize = 3;

    fn set_id(&mut self, id: usize) {
        self.id = id
    } 
}

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

#[derive(Debug)]
pub struct NodeCollection<T: Node> {
    nodes: Vec<T>,
}

impl<T: Node> NodeCollection<T> {
    fn new() -> Self {
        NodeCollection { nodes: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        NodeCollection { nodes: Vec::with_capacity(capacity) }
    }

    fn push_node(&mut self, n: T) {
        self.nodes.push(n);
    }

    fn sanitise(&mut self) {
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.set_id(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_collection() {
        let mut node_colletion: NodeCollection<Node1D> = NodeCollection::new();
        for i in 0..10 {
            let n = Node1D::new(i + 1, (i * 2) as f64);
            node_colletion.push_node(n);
        }
        println!("{:?}", node_colletion);
        node_colletion.sanitise();
        println!("{:?}", node_colletion);
    }
}