#![allow(dead_code)]

use std::io::{self, Write, BufWriter};
use std::fs::File;
use super::read::Plot3DBlock;

pub fn write_plot3d_binary(filename: &str, blocks: &[Plot3DBlock], double_precision: bool, big_endian: bool) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // write number of blocks as big endian u32
    let nblocks = blocks.len() as u32;
    match big_endian {
        true => writer.write_all(&nblocks.to_be_bytes())?,
        false => writer.write_all(&nblocks.to_le_bytes())?,
    }
    
    // write dimensions for each block
    for block in blocks {
        // write i, j, k as big-endian u32
        match big_endian {
            true => {
                writer.write_all(&(block.imax as u32).to_be_bytes())?;
                writer.write_all(&(block.jmax as u32).to_be_bytes())?;
                writer.write_all(&(block.kmax as u32).to_be_bytes())?;
            }
            false => {
                writer.write_all(&(block.imax as u32).to_le_bytes())?;
                writer.write_all(&(block.jmax as u32).to_le_bytes())?;
                writer.write_all(&(block.kmax as u32).to_le_bytes())?;
            }
        }
    }

    // write coordinate data for all blocks
    for block in blocks {
        write_plot3d_block_binary(&mut writer, block, double_precision, big_endian)?;
    }
    
    Ok(())
}

fn write_plot3d_block_binary<W: Write>(writer: &mut W, block: &Plot3DBlock, double_precision: bool, big_endian: bool) -> io::Result<()> {
    // write x coordinates
    for value in &block.x {
        if double_precision {
            match big_endian {
                true => writer.write_all(&value.to_be_bytes())?,
                false => writer.write_all(&value.to_le_bytes())?, 
            }
        } else {
            match big_endian {
                true => writer.write_all(&(*value as f32).to_be_bytes())?,
                false => writer.write_all(&(*value as f32).to_le_bytes())?,
            }
        }
    }

    // write y coordinates
    for value in &block.y {
        if double_precision {
            match big_endian {
                true => writer.write_all(&value.to_be_bytes())?,
                false => writer.write_all(&value.to_le_bytes())?, 
            }
        } else {
            match big_endian {
                true => writer.write_all(&(*value as f32).to_be_bytes())?,
                false => writer.write_all(&(*value as f32).to_le_bytes())?,
            }
        }
    }
    
    // write z coordinates
    for value in &block.z {
        if double_precision {
            match big_endian {
                true => writer.write_all(&value.to_be_bytes())?,
                false => writer.write_all(&value.to_le_bytes())?, 
            }
        } else {
            match big_endian {
                true => writer.write_all(&(*value as f32).to_be_bytes())?,
                false => writer.write_all(&(*value as f32).to_le_bytes())?,
            }
        }
    }

    Ok(())
}