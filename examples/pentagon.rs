use graph_solver::*;

// Notice that edges starts with `2`.
const EDGE: Color = 2;

fn main() {
    let mut g = Graph::new();

    // Create a node pattern.
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![
            Constraint {edge: EDGE, node: 0},
            Constraint {edge: EDGE, node: 0},
        ]
    };

    for _ in 0..5 {g.push(a.clone())}

    let solve_settings = SolveSettings::new();
    if let Some(solution) = g.solve(solve_settings) {
        // solution.puzzle.print();
        println!("{}", solution.puzzle.graphviz(
            "sfdp",
            &["black"],
            &["black"]
        ));
    }
}
