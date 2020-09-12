use graph_solver::*;

// Notice that edges starts with `2`.
const RED: Color = 2;

fn main() {
    let mut g = Graph::new();
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![Constraint {edge: RED, node: 1}]
    };
    let b = Node {
        color: 1,
        self_connected: false,
        edges: vec![Constraint {edge: RED, node: 0}]
    };
    g.push(a);
    g.push(b);

    let solve_settings = SolveSettings::new();
    if let Some(solution) = g.solve(solve_settings) {
        solution.puzzle.print();
    }
}
