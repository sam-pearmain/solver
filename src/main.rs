mod case;
mod fdm;
mod grid;
mod plot3d;

use grid::grid::Block;

fn main() -> Result<(), &'static str> {
    let block = Block::from_plot3d("double-wedge-fine.p3d")
        .map_err(|_| "could not create block")?;
    println!("{}", block);
    Ok(())
}
