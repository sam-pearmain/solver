#![allow(dead_code)]

use crate::meshing::vtk::VtkFile;

use super::{elements::{Element, ElementCollection}, nodes::{Node, NodeCollection}};

pub struct ConnectivityMatrix {}

pub struct Mesh<N, E>
where 
    N: Node,
    E: Element,
{
    nodes: NodeCollection<N>,
    elements: ElementCollection<E>,
    connectivity_matrix: ConnectivityMatrix,
}