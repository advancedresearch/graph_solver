use graph_solver::*;

// Notice that edges starts with `2`.
const EDGE: Color = 2;

fn main() {
    let mut g = Graph::new();

    let edge = Constraint {edge: EDGE, node: 0};

    // Create a node pattern.
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![edge; 2],
    };
    let b = Node {
        color: 0,
        self_connected: false,
        edges: vec![edge; 3]
    };
    let c = Node {
        color: 0,
        self_connected: false,
        edges: vec![edge; 4]
    };

    for _ in 0..4 {g.push(a.clone())}
    for _ in 0..4 {g.push(b.clone())}
    g.push(c);
    g.no_triangles = true;
    g.meet_quad = true;

    for i in 0..4 {g.set((i, 8), 1)}
    g.set((1, 5), 1);

    let solve_settings = SolveSettings::new();
    if let Some(solution) = g.solve(solve_settings) {
        // solution.puzzle.print();
        println!("{}", solution.puzzle.graphviz(
            "sfdp",
            &["black,fontcolor=white"],
            &["black"]
        ));
    } else {
        eprintln!("<no solution>");
    }
}
