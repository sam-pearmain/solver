#![allow(dead_code)]

use crate::meshing::vtk::VtkFile;

use super::{elements::{Element, ElementCollection}, nodes::{Node, NodeCollection}};

pub struct ConnectivityMatrix<E: Element> {
    
}

pub struct Mesh<N: Node, E: Element> {
    nodes: NodeCollection<N>,
    elements: ElementCollection<E>,
    connectivity_matrix: ConnectivityMatrix,
}

impl<N: Node, E: Element> Mesh<N, E> {
    fn new() -> Self {
        Mesh {
            nodes: NodeCollection::new(),
            elements: ElementCollection::new(),
            connectivity_matrix: ConnectivityMatrix::new(),
        }
    }
}