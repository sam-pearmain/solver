fn main() {
    // problem settings
    let n_elements = 9; // number of elements per direction, so nodes = n_elements + 1
    let n_nodes = n_elements + 1;
    let n_total = n_nodes * n_nodes; // total number of nodes

    // initialize solution vector for grid values (flattened 2d array)
    let mut u: Vec<f64> = vec![0.0; n_total];
    let mut u_next: Vec<f64> = u.clone();

    // apply dirichlet boundary conditions:
    // here we set u = 0 on the left and bottom boundaries and u = 1 on the right and top boundaries.
    for j in 0..n_nodes {
        for i in 0..n_nodes {
            let idx = j * n_nodes + i;
            if i == 0 { 
                // left boundary
                u[idx] = 0.0;
                u_next[idx] = 0.0;
            } else if i == n_nodes - 1 {
                // right boundary
                u[idx] = 1.0;
                u_next[idx] = 1.0;
            }
            if j == 0 {
                // bottom boundary
                u[idx] = 0.0;
                u_next[idx] = 0.0;
            } else if j == n_nodes - 1 {
                // top boundary
                u[idx] = 1.0;
                u_next[idx] = 1.0;
            }
        }
    }

    // parameters for the jacobi iterative solver
    let tol: f64 = 1e-10;
    let max_iter: i32 = 10000;
    let mut iter: i32 = 0;

    // jacobi iteration on interior nodes only
    loop {
        let mut diff: f64 = 0.0;
        // loop over interior grid points
        for j in 1..(n_nodes - 1) {
            for i in 1..(n_nodes - 1) {
                let idx = j * n_nodes + i;
                // compute average of four neighbours (up, down, left, right)
                let up = u[(j - 1) * n_nodes + i];
                let down = u[(j + 1) * n_nodes + i];
                let left = u[j * n_nodes + (i - 1)];
                let right = u[j * n_nodes + (i + 1)];
                u_next[idx] = (up + down + left + right) / 4.0;
                diff = diff.max((u_next[idx] - u[idx]).abs());
            }
        }
        u = u_next.clone();
        iter += 1;
        if diff < tol || iter >= max_iter {
            break;
        }
    }

    println!("jacobi iterative solution ({} iterations):", iter);
    // print the computed 2d solution grid
    for j in 0..n_nodes {
        for i in 0..n_nodes {
            print!("{:.4} ", u[j * n_nodes + i]);
        }
        println!();
    }
}