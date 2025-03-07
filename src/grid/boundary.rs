#![allow(dead_code)]

#[derive(Debug)]
pub enum Boundary {
    IMin,
    IMax,
    JMin, 
    JMax,
    KMin,
    KMax,
}

#[derive(Debug)]
pub enum BoundaryType {
    Wall,
    VelocityInlet,
    PressureOutlet,
}