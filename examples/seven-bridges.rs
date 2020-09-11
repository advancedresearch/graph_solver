/*
Seven Bridges of Königsberg
https://en.wikipedia.org/wiki/Seven_Bridges_of_K%C3%B6nigsberg
*/

use graph_solver::*;

fn main() {
    let mut g = Graph::new();

    let black_edge = Constraint {node: 0, edge: 2};
    let red_edge = Constraint {node: 0, edge: 3};
    let f = |black, red| Node {
        color: 0,
        self_connected: false,
        edges: {
            let mut res = vec![];
            for _ in 0..black {res.push(black_edge)}
            for _ in 0..red {res.push(red_edge)}
            res
        }
    };
    g.push(f(1, 1));
    g.push(f(2, 1));
    g.push(f(1, 1));
    g.push(f(1, 2));
    g.push(f(1, 3));
    g.push(f(0, 3));
    g.push(f(1, 1));
    g.push(f(2, 1));
    g.push(f(1, 1));

    g.set((0, 1), 2);
    g.set((1, 2), 2);
    g.set((0, 2), 1);
    g.set((1, 4), 3);
    g.set((2, 3), 1);
    g.set((2, 4), 1);
    g.set((3, 5), 1);
    g.set((3, 7), 1);
    g.set((3, 4), 2);

    g.connected = true;

    let solve_settings = SolveSettings::new()
        .debug(false)
        .sleep_ms(1000);
    if let Some(solution) = g.solve(solve_settings) {
        // solution.puzzle.print();
        println!("{}", solution.puzzle.graphviz(
            "sfdp",
            &["white"],
            &["black", "red"]
        ));
    } else {
        eprintln!("<no solution>");
    }
}
