#![allow(dead_code)]

use super::elements::{Element, ElementCollection};
use super::nodes::{Node, NodeCollection};

pub struct Mesh<N: Node, E: Element<N>> {
    nodes: NodeCollection<N>,
    elements: ElementCollection<N, E>,
    connectivity: Vec<Vec<usize>>,
}

impl<N: Node, E: Element<N>> Mesh<N, E> {
    fn new() -> Self {
        Mesh {
            nodes: NodeCollection::new(),
            elements: ElementCollection::new(),
            connectivity: Vec::new(),
        }
    }
}