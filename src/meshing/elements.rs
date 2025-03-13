#![allow(dead_code)]

use std::{marker::PhantomData, rc::Rc};
use super::nodes::{Node, Node1D};

pub trait Element<N: Node> { 
    const DOF: usize; // degrees of freedom

    fn set_id(&mut self, id: usize);
    fn volume(&self) -> f64;
    fn nodes(&self) -> Vec<Rc<N>>;
}

#[derive(Debug)]
pub struct LineElement {
    id: usize, 
    n1: Rc<Node1D>,
    n2: Rc<Node1D>,
}

impl Element<Node1D> for LineElement { 
    const DOF: usize = 2; 

    fn volume(&self) -> f64 {
        (self.n1.x - self.n2.x).abs()
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn nodes(&self) -> Vec<Rc<Node1D>> {
        vec![self.n1.clone(), self.n2.clone()]
    }
}

impl LineElement {
    pub fn new(id: usize, n1: Rc<Node1D>, n2: Rc<Node1D>) -> Self {
        Self { id, n1, n2 }
    }
}

pub struct ElementCollection<N: Node, T: Element<N>> {
    elements: Vec<T>,
    _phantom: PhantomData<N>
}

impl<N: Node, E: Element<N>> ElementCollection<N, E> {
    pub fn new() -> Self {
        ElementCollection { elements: Vec::new(), _phantom: PhantomData }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ElementCollection { elements: Vec::with_capacity(capacity), _phantom: PhantomData }
    }

    pub fn push_element(&mut self, e: E) {
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
        let line = LineElement::new(0, Rc::new(n1), Rc::new(n2));
        println!("{:?}", line);
    }
}