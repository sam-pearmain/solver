pub enum BasisFunction {
    Linear,
}

fn main() {
    // linear fem settings: number of nodes = number of elements + 1
    let domain_length = 2.0;
    let n_elements = 9; // 9 elements => 10 nodes
    let n_nodes = n_elements + 1;

    // generate mesh with equally spaced nodes
    let mut x = vec![0.0; n_nodes];
    let dx = domain_length / (n_elements as f64);
    for i in 0..n_nodes {
        x[i] = i as f64 * dx;
    }

    // initialise global stiffness matrix and right-hand side vector
    let mut k_global = vec![vec![0.0; n_nodes]; n_nodes];
    let mut f_global = vec![0.0; n_nodes];

    // loop over each element and assemble the local contributions
    for i in 0..n_elements {
        // element length
        let h = x[i + 1] - x[i];

        let k_local = [
            [1.0 / h, -1.0 / h],
            [-1.0 / h, 1.0 / h],
        ];

        // assemble into the global stiffness matrix
        k_global[i][i] += k_local[0][0];
        k_global[i][i + 1] += k_local[0][1];
        k_global[i + 1][i] += k_local[1][0];
        k_global[i + 1][i + 1] += k_local[1][1];
    }

    // apply dirichlet boundary conditions: u(0)=0 and u(2)=1
    let bc_left = 0.0;
    let bc_right = 1.0;

    // modify the first row for left boundary condition
    k_global[0] = vec![0.0; n_nodes];
    k_global[0][0] = 1.0;
    f_global[0] = bc_left;

    // modify the last row for right boundary condition
    k_global[n_nodes - 1] = vec![0.0; n_nodes];
    k_global[n_nodes - 1][n_nodes - 1] = 1.0;
    f_global[n_nodes - 1] = bc_right;

    // solve the global system using a jacobi iterative method
    let n = n_nodes;
    let tol = 1e-10;
    let max_iter = 10000;
    let mut u = vec![0.0; n]; // initial guess
    let mut u_next = u.clone();
    let mut iter = 0;
    loop {
        // perform one iteration of jacobi update for each equation
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if j != i {
                    sigma += k_global[i][j] * u[j];
                }
            }
            u_next[i] = (f_global[i] - sigma) / k_global[i][i];
        }
        // compute maximum error between u and u_next for convergence
        let mut diff: f64 = 0.0;
        for i in 0..n {
            diff = diff.max((u_next[i] - u[i]).abs());
        }
        u = u_next.clone();
        iter += 1;
        if diff < tol || iter >= max_iter {
            break;
        }
    }

    println!("jacobi iterative solution ({} iterations): {:?}", iter, u);
}