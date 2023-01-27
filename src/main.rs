use itertools::iproduct;

use good_lp::{constraint, variable, variables, Constraint, default_solver};

fn idx(val: usize, row: usize, col: usize) -> usize {
    (val - 1) * 9 * 9 + row * 9 + col
}

fn main() {
    // iproduct!(1..=9, 0..9, 0..9)

    let nvars = 9 * 9 * 9;
    let mut choices = variables!();
    let vars = choices.add_vector(variable().binary(), nvars);

    let squares: Vec<Vec<(usize, usize)>> = iproduct!(0..3, 0..3)
        .map(|(x, y)| {
            iproduct!(0..3, 0..3)
                .map(|(i, j)| (3 * x + i, 3 * y + j))
                .collect()
        })
        .collect();

    println!("{:?}", squares);

    let mut constraints: Vec<Constraint> = Vec::with_capacity(4 * 9 * 9); // add input
    for (row, col) in iproduct!(0..9, 0..9) {
        // constraints.push(constraint!(
        //     (1..=9).map(|val| vars[idx(val, row, col)]).sum::<Vec<Variable>>() == 1
        // ));
        constraints.push(constraint!(
            vars[idx(1, row, col)]
                + vars[idx(2, row, col)]
                + vars[idx(3, row, col)]
                + vars[idx(4, row, col)]
                + vars[idx(5, row, col)]
                + vars[idx(6, row, col)]
                + vars[idx(7, row, col)]
                + vars[idx(8, row, col)]
                + vars[idx(9, row, col)]
                == 1
        ));
    }

    for val in 1..=9 {
        for row in 0..9 {
            constraints.push(constraint!(
                vars[idx(val, row, 1)]
                    + vars[idx(val, row, 2)]
                    + vars[idx(val, row, 3)]
                    + vars[idx(val, row, 4)]
                    + vars[idx(val, row, 5)]
                    + vars[idx(val, row, 6)]
                    + vars[idx(val, row, 7)]
                    + vars[idx(val, row, 8)]
                    + vars[idx(val, row, 9)]
                    == 1
            ));
        }
        for col in 0..9 {
            constraints.push(constraint!(
                vars[idx(val, 1, col)]
                    + vars[idx(val, 2, col)]
                    + vars[idx(val, 3, col)]
                    + vars[idx(val, 4, col)]
                    + vars[idx(val, 5, col)]
                    + vars[idx(val, 6, col)]
                    + vars[idx(val, 7, col)]
                    + vars[idx(val, 8, col)]
                    + vars[idx(val, 9, col)]
                    == 1
            ));
        }
	// TODO: squares
        // for sq in squares.iter() {
	//     constraint!(
	// 	for (i, j) in sq.iter()
	//     )
	// }

	// TODO: solve, see example in: https://docs.rs/good_lp/latest/good_lp/index.html
	// I think we can do something like below, but not sure what would be the argument to optimise
	// https://docs.rs/good_lp/latest/good_lp/variable/struct.ProblemVariables.html#method.optimise
	let mut solution = choices.optimise().using(default_solver);
	// loop over constraints
	
    }

    // test
    let vis = choices.display(&vars[0]);
    println!("{}", vis);
}
