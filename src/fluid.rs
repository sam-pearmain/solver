#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub enum FluidType {
    Air,
    Water,
}

#[derive(Debug, Clone, Copy)]
pub struct Fluid {
    fluid_type: FluidType,
    density: f64,
    gamma: f64,
    viscosity: f64,
}

impl Fluid {
    fn from_substance(fluid_type: FluidType) -> Self {
        match fluid_type {
            FluidType::Air => {
                Fluid {
                    fluid_type,
                    gamma: 1.4,
                    density: 1.225,
                    viscosity: 1.0,
                }
            }
            FluidType::Water => {
                todo!()
            }
        }
    }

    pub fn density(&self) -> f64 {
        self.density
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    pub fn viscosity(&self) -> f64 {
        self.viscosity
    }
}