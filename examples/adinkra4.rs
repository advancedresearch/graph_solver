use graph_solver::*;

// Notice that edges start with `2`.
const RED: Color = 2;
const RED_DASHED: Color = 3;
const GREEN: Color = 4;
const GREEN_DASHED: Color = 5;
const BLUE: Color = 6;
const BLUE_DASHED: Color = 7;
const ORANGE: Color = 8;
const ORANGE_DASHED: Color = 9;

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
            Constraint {edge: ORANGE, node: WHITE},
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
            Constraint {edge: ORANGE_DASHED, node: BLACK},
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
            Constraint {edge: ORANGE_DASHED, node: BLACK},
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
            Constraint {edge: ORANGE_DASHED, node: WHITE},
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
            Constraint {edge: ORANGE_DASHED, node: BLACK},
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
            Constraint {edge: ORANGE, node: WHITE},
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
            Constraint {edge: ORANGE, node: BLACK},
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
            Constraint {edge: ORANGE, node: WHITE},
        ]
    });
    // 8
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: ORANGE, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
            Constraint {edge: RED, node: BLACK},
            Constraint {edge: GREEN, node: BLACK},
        ]
    });
    // 9
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: BLUE_DASHED, node: WHITE},
            Constraint {edge: ORANGE_DASHED, node: WHITE},
            Constraint {edge: RED_DASHED, node: WHITE},
            Constraint {edge: GREEN_DASHED, node: WHITE},
        ]
    });
    // 10
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED_DASHED, node: BLACK},
            Constraint {edge: ORANGE, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
            Constraint {edge: GREEN, node: BLACK},
        ]
    });
    // 11
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: BLUE_DASHED, node: WHITE},
            Constraint {edge: ORANGE_DASHED, node: WHITE},
            Constraint {edge: RED, node: WHITE},
            Constraint {edge: GREEN, node: WHITE},
        ]
    });
    // 12
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: ORANGE, node: BLACK},
            Constraint {edge: GREEN_DASHED, node: BLACK},
            Constraint {edge: RED_DASHED, node: BLACK},
            Constraint {edge: BLUE_DASHED, node: BLACK},
        ]
    });
    // 13
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED_DASHED, node: WHITE},
            Constraint {edge: ORANGE, node: WHITE},
            Constraint {edge: GREEN, node: WHITE},
            Constraint {edge: BLUE, node: WHITE},
        ]
    });
    // 14
    g.push(Node {
        color: WHITE,
        self_connected: false,
        edges: vec![
            Constraint {edge: BLUE, node: BLACK},
            Constraint {edge: ORANGE_DASHED, node: BLACK},
            Constraint {edge: RED_DASHED, node: BLACK},
            Constraint {edge: GREEN, node: BLACK},
        ]
    });
    // 15
    g.push(Node {
        color: BLACK,
        self_connected: false,
        edges: vec![
            Constraint {edge: BLUE_DASHED, node: WHITE},
            Constraint {edge: RED_DASHED, node: WHITE},
            Constraint {edge: GREEN, node: WHITE},
            Constraint {edge: ORANGE_DASHED, node: WHITE},
        ]
    });

    // Uncomment for improved performance.
    /*
    g.set((0, 1), RED);
    g.set((0, 8), ORANGE);
    g.set((0, 2), GREEN);
    g.set((0, 4), BLUE_DASHED);
    g.set((2, 3), RED);
    g.set((2, 5), BLUE_DASHED);
    g.set((1, 3), GREEN_DASHED);
    g.set((1, 11), ORANGE_DASHED);
    g.set((8, 11), RED);
    g.set((12, 13), RED_DASHED);
    g.set((14, 15), RED_DASHED);
    g.set((3, 14), ORANGE_DASHED);
    g.set((5, 6), RED);
    g.set((10, 13), GREEN);
    g.set((1, 7), BLUE_DASHED);
    */

    // Require anticommutativity for every quad.
    g.commute_quad = Some(false);

    let solve_settings = SolveSettings::new(); // .debug(true); // .sleep_ms(1000);
    if let Some(solution) = g.solve(solve_settings) {
        // solution.puzzle.print();
        // let nodes = &["black,fontcolor=white,label=\"\"", "white,label=\"\""];
        let nodes = &["black,fontcolor=white", "white"];
        let edges = &[
            "red", "red,style=dashed",
            "green", "green,style=dashed",
            "blue", "blue,style=dashed",
            "orange", "orange,style=dashed",
        ];
        println!("{}", solution.puzzle.graphviz("neato", nodes, edges));
    } else {
        eprintln!("<no solution>");
    }
}
