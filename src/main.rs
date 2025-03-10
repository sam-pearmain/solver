mod meshing;

use meshing::mesh::Mesh;

fn main() -> Result<(), &'static str> {
    let mut mesh = Mesh::from_vtk("structured-double-wedge.vtk")?;
    mesh.build_elements()?;
    mesh.plot_dbg()?;
    Ok(())
}
