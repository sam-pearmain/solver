mod case;
mod fdm;
mod fluid;
mod grid;
mod plot3d;

use grid::grid::Block;

fn main() -> Result<(), &'static str> {
    let block = Block::from_plot3d("double-wedge-fine.p3d")
        .map_err(|_| "could not create block")?;
    block.export_txt("block.txt")
        .map_err(|_| "what")?;
    Ok(())
}
