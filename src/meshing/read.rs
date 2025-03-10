use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct VtkFile {
    pub version: String,
    pub points: Vec<[f64; 3]>,
    pub cells: Vec<Vec<usize>>,
}

pub fn read_vtk(file_path: &str) -> Result<VtkFile, &'static str> {
    let file = File::open(file_path).map_err(|_| "failed to open file")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // read the first line to get the vtk version info
    let version = if let Some(line) = lines.next() {
        let line = line.map_err(|_| "failed to read version line")?;
        line.trim().to_string()  // e.g., "# vtk DataFile Version 4.2"
    } else {
        return Err("file is empty");
    };

    let mut points: Vec<[f64; 3]> = Vec::new();
    let mut cells: Vec<Vec<usize>> = Vec::new();

    // find the points section and read coordinates
    while let Some(line) = lines.next() {
        let line = line.map_err(|_| "failed to read line")?;
        if line.trim().starts_with("POINTS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err("invalid points header, vtk file likely corrupt");
            }
            let n_points: usize = parts[1].parse().map_err(|_| "failed to parse number of points")?;
            // read point coordinates; the coordinates may be split over several lines
            // so we accumulate until we have 3 * n_points values (each point has x, y, z value)
            let mut coords: Vec<f64> = Vec::with_capacity(3 * n_points);
            while coords.len() < 3 * n_points {
                if let Some(line) = lines.next() {
                    let line = line.map_err(|_| "failed to read coordinates")?;
                    for token in line.split_whitespace() {
                        if let Ok(val) = token.parse::<f64>() {
                            coords.push(val);
                        }
                    }
                } else {
                    break;
                }
            }
            if coords.len() < 3 * n_points {
                return Err("insufficient point data");
            }
            for i in 0..n_points {
                points.push([coords[3 * i], coords[3 * i + 1], coords[3 * i + 2]]);
            }
            break;
        }
    }

    // continue reading until cells section is found
    while let Some(line) = lines.next() {
        let line = line.map_err(|_| "failed to read line")?;
        if line.trim().starts_with("CELLS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err("invalid cells header, vtk file likely corrupt");
            }
            let n_cells: usize = parts[1].parse().map_err(|_| "failed to parse number of cells")?;
            // read each cell connectivity; each cell line begins with a count of points for that cell
            for _ in 0..n_cells {
                if let Some(line) = lines.next() {
                    let line = line.map_err(|_| "failed to read cell line")?;
                    let tokens: Vec<usize> = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    if tokens.is_empty() {
                        return Err("empty cell line encountered");
                    }
                    let n_nodes_in_cell = tokens[0];
                    if tokens.len() < n_nodes_in_cell {
                        return Err("insufficient cell index data");
                    }
                    let connectivity = tokens[1..(n_nodes_in_cell + 1)].to_vec();
                    cells.push(connectivity);
                }
            }
            break;
        }
    }

    Ok(VtkFile { version, points, cells })
}