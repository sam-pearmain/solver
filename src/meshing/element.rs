#![allow(unused)]

pub enum ElementType {
    Point, 
    Segment, 
    Triangle, 
    Quadrilateral, 
    Tetrahedron, 
    Hexahedron, 
}

pub trait Element {
    type Geometry: BaseGeometry;
    /// Returns the elements type
    fn get_type() -> ElementType;
    /// Returns the type of geometry the element is based upon
    fn get_geometry_type() -> Self::Geometry;
    /// Returns a vector storing the indecies of the vertices
    fn get_vertices() -> Vec<usize>;

    fn set_vertices(vertices: Vec<usize>);

    fn get_n_vertices() -> usize { 0 }

    fn get_n_edges() -> usize { 0 }

    fn get_n_faces() -> usize { 0 }
}