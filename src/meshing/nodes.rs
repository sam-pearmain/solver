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
    fn set_id(&mut self, id: usize); 
    fn x(&self) -> Option<f64>;
    fn y(&self) -> Option<f64>;
    fn z(&self) -> Option<f64>;
    fn dimensions(&self) -> usize;
}

impl Node for Node1D { 
    fn set_id(&mut self, id: usize) {
        self.id = id
    } 

    fn x(&self) -> Option<f64> {
        Some(self.x)
    }

    fn y(&self) -> Option<f64> {
        None
    }

    fn z(&self) -> Option<f64> {
        None
    }

    fn dimensions(&self) -> usize {
        1
    }
}

impl Node for Node2D { 
    fn set_id(&mut self, id: usize) {
        self.id = id
    }

    fn x(&self) -> Option<f64> {
        Some(self.x)
    }

    fn y(&self) -> Option<f64> {
        Some(self.y)
    }

    fn z(&self) -> Option<f64> {
        None
    }

    fn dimensions(&self) -> usize {
        2
    }
}

impl Node for Node3D {
    fn set_id(&mut self, id: usize) {
        self.id = id
    } 

    fn x(&self) -> Option<f64> {
        Some(self.x)
    }

    fn y(&self) -> Option<f64> {
        Some(self.y)
    }

    fn z(&self) -> Option<f64> {
        Some(self.z)
    }

    fn dimensions(&self) -> usize {
        3
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
    fn dimensions(&self) -> usize;
}

impl<T: Node> Dimensioned for T {
    fn dimensions(&self) -> usize {
        self.dimensions()
    }
}

#[derive(Debug)]
pub struct NodeCollection<T: Node> {
    pub nodes: Vec<T>,
}

impl<T: Node> NodeCollection<T> {
    pub fn new() -> Self {
        NodeCollection { nodes: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        NodeCollection { nodes: Vec::with_capacity(capacity) }
    }

    pub fn from_nodes(nodes: Vec<T>) -> Self {
        NodeCollection { nodes }
    }

    pub fn push_node(&mut self, n: T) {
        self.nodes.push(n);
    }

    pub fn sanitise(&mut self) {
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.set_id(i);
        }
    }

    pub fn get_n_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, index: usize) -> &T {
        &self.nodes[index]
    }

    pub fn get_x_vec(&self) -> Option<Vec<f64>> {
        let mut x_vec = Vec::new();
        for node in self.nodes.iter() {
            if let Some(x) = node.x() {
                x_vec.push(x);
            }
        }
        if !x_vec.is_empty() { Some(x_vec) } else { None }
    }

    pub fn get_y_vec(&self) -> Option<Vec<f64>> {
        let mut y_vec = Vec::new();
        for node in self.nodes.iter() {
            if let Some(y) = node.y() {
                y_vec.push(y);
            }
        }
        if !y_vec.is_empty() { Some(y_vec) } else { None }
    }

    pub fn get_z_vec(&self) -> Option<Vec<f64>> {
        let mut z_vec = Vec::new();
        for node in self.nodes.iter() {
            if let Some(z) = node.z() {
                z_vec.push(z);
            }
        }
        if !z_vec.is_empty() { Some(z_vec) } else { None }
    }
}

impl<T: Node> Dimensioned for NodeCollection<T> {
    fn dimensions(&self) -> usize {
        self.nodes.first().map(|n| n.dimensions()).unwrap_or(0)
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