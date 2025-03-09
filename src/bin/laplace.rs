pub enum BasisFunction {
    Linear,
    Quadratic,
}

fn main() {
    // choose the basis function: linear or quadratic
    let basis = BasisFunction::Quadratic; // change as desired

    match basis {
        BasisFunction::Linear => run_linear_fem(),
        BasisFunction::Quadratic => run_quadratic_fem(),
    }
}

fn run_linear_fem() {
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

    // solve the global system using gaussian elimination
    let u = gauss_elimination(&mut k_global, &mut f_global);
    println!("linear fem solution: {:?}", u);
}

fn run_quadratic_fem() {
    // quadratic fem settings: number of nodes = 2 * number of elements + 1
    let domain_length = 2.0;
    let n_elements = 9;
    let n_nodes = 2 * n_elements + 1;

    // generate mesh with equally spaced nodes
    let mut x = vec![0.0; n_nodes];
    let dx = domain_length / ((n_nodes - 1) as f64);
    for i in 0..n_nodes {
        x[i] = i as f64 * dx;
    }

    // initialise global stiffness matrix and right-hand side vector
    let mut k_global = vec![vec![0.0; n_nodes]; n_nodes];
    let mut f_global = vec![0.0; n_nodes];

    // loop over each element (local nodes: i0, i1, i2) and assemble the local contributions
    for e in 0..n_elements {
        let i0 = 2 * e;
        let i1 = 2 * e + 1;
        let i2 = 2 * e + 2;

        // element length measured between first and last node
        let h = x[i2] - x[i0];
        // factor for the quadratic stiffness matrix
        let factor = 1.0 / (3.0 * h);
        // local stiffness matrix for quadratic elements
        // (coefficients are derived by integrating the derivatives of the quadratic shape functions)
        let k_local = [
            [7.0 * factor, -8.0 * factor, 1.0 * factor],
            [-8.0 * factor, 16.0 * factor, -8.0 * factor],
            [1.0 * factor, -8.0 * factor, 7.0 * factor],
        ];

        // assemble local stiffness matrix into the global stiffness matrix
        k_global[i0][i0] += k_local[0][0];
        k_global[i0][i1] += k_local[0][1];
        k_global[i0][i2] += k_local[0][2];

        k_global[i1][i0] += k_local[1][0];
        k_global[i1][i1] += k_local[1][1];
        k_global[i1][i2] += k_local[1][2];

        k_global[i2][i0] += k_local[2][0];
        k_global[i2][i1] += k_local[2][1];
        k_global[i2][i2] += k_local[2][2];
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

    // solve the global system using gaussian elimination
    let u = gauss_elimination(&mut k_global, &mut f_global);
    println!("quadratic fem solution: {:?}", u);
}

// a simple gaussian elimination solver for ax = b,
// this sample does not include pivoting.
fn gauss_elimination(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> Vec<f64> {
    let n = b.len();

    // forward elimination
    for i in 0..n {
        let pivot = a[i][i];
        assert!(pivot.abs() > 1e-12, "zero pivot encountered");
        for j in i..n {
            a[i][j] /= pivot;
        }
        b[i] /= pivot;
        for k in i+1..n {
            let factor = a[k][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }

    // back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in i+1..n {
            x[i] -= a[i][j] * x[j];
        }
    }
    x
}