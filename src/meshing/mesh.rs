#![allow(dead_code)]

pub struct Node {
    id: usize,
    x: f64,
    y: f64,
}

pub struct Element<'a> {
    id: usize, 
    nodes: Vec<&'a Node>,
}

pub struct Mesh<'a> {
    // ndims: Dimensions,
    nodes: Vec<Node>,
    elements: Vec<Element<'a>>
}

impl Mesh<'_> {
    pub fn from_vtk(file_path: &str) -> Result<Self, &'static str> {
        todo!();
    }
}