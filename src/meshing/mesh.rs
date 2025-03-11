#![allow(dead_code)]

use plotters::prelude::*;
use crate::meshing::vtk::VtkFile;
use super::{elements::{Element, ElementCollection, TriangleElement}, nodes::{Node, Node2D, Node3D, NodeCollection}};

pub struct Mesh<N: Node, E: Element> {
    nodes: NodeCollection<N>,
    elements: ElementCollection<E>,
    connectivity: Vec<Vec<usize>>,
}

impl<N: Node, E: Element> Mesh<N, E> {
    fn new() -> Self {
        Mesh {
            nodes: NodeCollection::new(),
            elements: ElementCollection::new(),
            connectivity: Vec::new(),
        }
    }

    fn from_vtk_2d(vtk_file: &str) -> Result<Mesh<Node2D, E>, &'static str> {
        let vtk = VtkFile::read_vtk(vtk_file)?;
        let points = vtk.points;
        let mut nodes: Vec<Node2D> = Vec::with_capacity(points.len());
        
        for (id, pt) in points.iter().enumerate() {
            let n = Node2D::new(id, pt[0], pt[1]);
            nodes.push(n);
        }
        
        let node_collection = NodeCollection::from_nodes(nodes);
        
        Ok(Mesh {
            nodes: node_collection,
            elements: ElementCollection::new(),
            connectivity: vtk.cells,
        })
    }

    fn from_vtk_3d(vtk_file: &str) -> Result<Mesh<Node3D, E>, &'static str> {
        let vtk = VtkFile::read_vtk(vtk_file)?;
        let points = vtk.points;
        let mut nodes: Vec<Node3D> = Vec::with_capacity(points.len());
        
        for (id, pt) in points.iter().enumerate() {
            let n = Node3D::new(id, pt[0], pt[1], pt[3]);
            nodes.push(n);
        }
        
        let node_collection = NodeCollection::from_nodes(nodes);
        
        Ok(Mesh {
            nodes: node_collection,
            elements: ElementCollection::new(),
            connectivity: vtk.cells,
        })
    }
}

impl<E: Element> Mesh<Node2D, E> {
    fn plot_dbg(&self, filepath: &str) -> Result<(), &'static str> {
        let root = BitMapBackend::new(filepath, (2560, 1440)).into_drawing_area();
        root.fill(&WHITE).map_err(|_| "failed to fill")?;

        let x_vec = self.nodes.get_x_vec().ok_or("failed to get x_vec")?;
        let y_vec = self.nodes.get_y_vec().ok_or("failed to get y_vec")?;

        let x_min = x_vec.iter().copied().fold(f64::INFINITY, f64::min) - 1.0;
        let x_max = x_vec.iter().copied().fold(f64::NEG_INFINITY, f64::max) + 1.0;
        let y_min = y_vec.iter().copied().fold(f64::INFINITY, f64::min) - 1.0;
        let y_max = y_vec.iter().copied().fold(f64::NEG_INFINITY, f64::max) + 1.0;
        
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption("mesh plot", ("consolas", 40))
            .x_label_area_size(20)
            .y_label_area_size(20)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|_| "failed to build chart")?;
        chart.configure_mesh().draw().map_err(|_| "failed to draw mesh grid")?;

        chart.draw_series(
            self.nodes.nodes.iter().map(|node| {
                Circle::new((node.x, node.y), 5, ShapeStyle::from(&BLUE).filled())
            })
        ).map_err(|_| "failed to draw nodes")?;
        
        // Iterate over every element in the mesh and plot its connectivity.
        // We assume that self.elements has an .iter() method.
        for element in self.elements.iter() {
            // Use the iter_nodes() method to get an iterator over &dyn Node.
            let points: Vec<(f64, f64)> = element.iter_nodes()
                .filter_map(|node| match (node.x(), node.y()) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                })
                .collect();
                
            if points.len() >= 2 {
                let mut polyline = points.clone();
                // For shapes with more than two nodes, close the polygon.
                if points.len() > 2 {
                    polyline.push(points[0]);
                }
                chart.draw_series(LineSeries::new(polyline, &RED))
                    .map_err(|_| "failed to draw element connectivity")?;
            }
        }
        
        // Present the drawing.
        root.present().map_err(|_| "failed to present drawing area")?;
        Ok(())
    } 
}

impl<'a> Mesh<Node2D, TriangleElement<'a>> {
    fn build_elements(&mut self) {
        
    }
}