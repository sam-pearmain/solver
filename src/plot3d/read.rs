#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Debug)]
pub struct Plot3DBlock {
    pub imax: i32, 
    pub jmax: i32, 
    pub kmax: i32, 
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}

pub fn read_plot3d_ascii(filename: &str) -> io::Result<Vec<Plot3DBlock>> {
    if !filename.to_lowercase().ends_with(".xyz") && !filename.to_lowercase().ends_with(".p3d") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "input file must have .xyz extension"
        ));
    }
    
    let input_file = File::open(filename)?;
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    // read number of blocks and validate number of blocks
    let nblocks: i32 = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "input file is empty"))??
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid number of blocks, file possibly corrupt"))?;
    
    if nblocks <= 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid number of blocks: {0}", nblocks)
        ));
    }

    // create a vec of plot3dblocks of length nblocks please
    let mut blocks: Vec<Plot3DBlock> = Vec::with_capacity(nblocks as usize);
    
    for _ in 0..nblocks {
        let ijk: Vec<i32> = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "missing block dimensoins"))??
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid block dimensoins"))?;

        if ijk.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected 3 dimensions, got {}. corrupt data", ijk.len())
            ));
        }
        blocks.push(Plot3DBlock {
            imax: ijk[0],
            jmax: ijk[1],
            kmax: ijk[2],
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
        });
    }

    for block in &mut blocks {
        block.x = read_plot3d_chunk_ascii(&mut lines, block.imax, block.jmax, block.kmax)?;
        block.y = read_plot3d_chunk_ascii(&mut lines, block.imax, block.jmax, block.kmax)?;
        block.z = read_plot3d_chunk_ascii(&mut lines, block.imax, block.jmax, block.kmax)?;
    }

    Ok(blocks)
}

pub fn read_plot3d_binary() {
    todo!()
}

fn read_plot3d_chunk_ascii(lines: &mut Lines<BufReader<File>>, imax: i32, jmax: i32, kmax: i32) -> io::Result<Vec<f64>> {
    let total_size = (imax * jmax * kmax) as usize;
    let mut values: Vec<f64> = Vec::with_capacity(total_size);

    while values.len() < total_size {
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "eof while reading block"))??;

        for num_str in line.split_whitespace() {
            let value = num_str
                .parse::<f64>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid number in block data"))?;
            values.push(value);
        }
    }

    if values.len() != total_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected {} values, got {}", total_size, values.len())
        ));
    }

    Ok(values)
}

fn read_plot3d_chunk_binary() {
    todo!()
}