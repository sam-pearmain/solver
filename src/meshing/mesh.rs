#![allow(dead_code)]

use std::rc::Rc;
use crate::meshing::vtk::VtkFile;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

pub struct TriangleElement {
    pub id: usize, 
    pub nodes: Vec<Rc<Node>>,
}

pub struct Mesh {
    pub nodes: Vec<Rc<Node>>,
    pub connectivity: Vec<Vec<usize>>,
    pub elements: Vec<TriangleElement>,
}

impl Mesh {
    pub fn from_vtk(file_path: &str) -> Result<Self, &'static str> {
        let vtk_file = VtkFile::read_vtk(file_path)?;
        
        let tri_connectivity: Vec<Vec<usize>> = vtk_file.cells.into_iter()
            .filter(|c| c.len() == 3)
            .collect();

        // create nodes as Rc<Node> from the vtk points (using x and y only)
        let nodes: Vec<Rc<Node>> = vtk_file.points.iter().enumerate().map(|(i, p)| {
            Rc::new(Node { id: i, x: p[0], y: p[1] })
        }).collect();

        Ok(Mesh { nodes, connectivity: tri_connectivity, elements: Vec::new() })
    }

    pub fn build_elements(&mut self) -> Result<(), &'static str> {
        for (elem_id, connectivity) in self.connectivity.iter().enumerate() {
            if connectivity.len() != 3 {
                return Err("non-triangular connectivity encountered");
            }
            let n0 = self.nodes[connectivity[0]].clone();
            let n1 = self.nodes[connectivity[1]].clone();
            let n2 = self.nodes[connectivity[2]].clone();

            self.elements.push(TriangleElement { id: elem_id, nodes: vec![n0, n1, n2] });
        }
        Ok(())
    }

    pub fn plot_dbg(&self) -> Result<(), &'static str> {
        use plotters::prelude::*;

        // define output file and drawing area
        let output_file = "mesh.png";
        let drawing_area = BitMapBackend::new(output_file, (2560, 1440))
            .into_drawing_area();
        drawing_area.fill(&WHITE).map_err(|_| "failed to fill background")?;

        // compute the boundaries for the chart from the nodes
        let xs: Vec<f64> = self.nodes.iter().map(|n| n.x).collect();
        let ys: Vec<f64> = self.nodes.iter().map(|n| n.y).collect();
        let x_min = xs.iter().cloned().fold(f64::INFINITY, f64::min);
        let x_max = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let y_min = ys.iter().cloned().fold(f64::INFINITY, f64::min);
        let y_max = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // build a 2D chart with the computed boundaries
        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(10)
            .caption("mesh plot", ("consolas", 20))
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|_| "failed to build chart")?;
        chart.configure_mesh().draw().map_err(|_| "failed to draw mesh grid")?;

        // plot all mesh points as blue circles
        chart.draw_series(
            self.nodes.iter().map(|node| {
                Circle::new((node.x, node.y), 0.5, ShapeStyle::from(&BLACK).filled())
            })
        ).map_err(|_| "failed to draw points")?;

        // plot connectivity lines for each triangle element (joining in a cycle)
        for conn in &self.connectivity {
            if conn.len() == 3 {
                let mut pts: Vec<(f64, f64)> = conn.iter().filter_map(|&idx| {
                    self.nodes.get(idx).map(|node| (node.x, node.y))
                }).collect();
                // complete the triangle by connecting last point to first
                if pts.len() == 3 {
                    pts.push(pts[0]);
                    chart.draw_series(LineSeries::new(pts, &BLACK))
                        .map_err(|_| "failed to draw connectivity line")?;
                }
            }
        }

        drawing_area.present().map_err(|_| "failed to present drawing area")?;
        Ok(())
    }
}