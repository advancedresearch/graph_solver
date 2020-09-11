use graph_solver::*;

// Notice that edges start with `2`.
const RED: Color = 2;
const RED_DASHED: Color = 3;
const GREEN: Color = 4;
const GREEN_DASHED: Color = 5;
const BLUE: Color = 6;
const BLUE_DASHED: Color = 7;

const BLACK: Color = 0;
const WHITE: Color = 1;

fn main() {
    let mut g = Graph::new();
    // 0
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: WHITE},
            Constraint {edge: GREEN, node: WHITE},
            Constraint {edge: BLUE_DASHED, node: WHITE},
        ]
    });
    // 1
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: BLACK},
            Constraint {edge: GREEN_DASHED, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
        ]
    });
    // 2
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: GREEN, node: BLACK},
            Constraint {edge: RED, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
        ]
    });
    // 3
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: WHITE},
            Constraint {edge: GREEN_DASHED, node: WHITE},
            Constraint {edge: BLUE, node: WHITE},
        ]
    });
    // 4
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED_DASHED, node: BLACK},
            Constraint {edge: GREEN_DASHED, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
        ]
    });
    // 5
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: WHITE},
            Constraint {edge: GREEN_DASHED, node: WHITE},
            Constraint {edge: BLUE_DASHED, node: WHITE},
        ]
    });
    // 6
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: BLACK},
            Constraint {edge: GREEN_DASHED, node: BLACK},
            Constraint {edge: BLUE, node: BLACK},
        ]
    });
    // 7
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED_DASHED, node: WHITE},
            Constraint {edge: GREEN_DASHED, node: WHITE},
            Constraint {edge: BLUE_DASHED, node: WHITE},
        ]
    });

    // Require anticommutativity for every quad.
    g.commute_quad = Some(false);

    let solve_settings = SolveSettings::new(); // .debug(true); // .sleep_ms(1000);
    if let Some(solution) = g.solve(solve_settings) {
        // solution.puzzle.print();
        let nodes = &["black,fontcolor=white,label=\"\"", "white,label=\"\""];
        let edges = &[
            "red", "red,style=dashed",
            "green", "green,style=dashed",
            "blue", "blue,style=dashed",
        ];
        println!("{}", solution.puzzle.graphviz("sfdp", nodes, edges));
    } else {
        eprintln!("<no solution>");
    }
}
