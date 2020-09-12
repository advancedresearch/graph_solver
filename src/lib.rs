#![deny(missing_docs)]

//! # Graph Solver
//! An undirected graph constraint solver for node and edge colors.
//!
//! Brought to you by the [AdvancedResearch](https://github.com/advancedresearch) community!
//!
//! - If you are looking for a generic solver that does not remove facts,
//! see [monotonic_solver](https://github.com/advancedresearch/monotonic_solver)
//! - If you are looking for a generic solver that can remove facts,
//! see [linear_solver](https://github.com/advancedresearch/linear_solver)
//! - If you are looking for a brute-force automated theorem prover for classical and path semantical logic,
//! see [pocket_prover](https://github.com/advancedresearch/pocket_prover)
//!
//! ### Motivation
//!
//! Graph solving is used to find visual representations of various algebras,
//! such as [Adinkra diagrams](https://en.wikipedia.org/wiki/Adinkra_symbols_(physics)),
//! but where the rules for generating the graphs efficiently is unknown.
//!
//! ![adinkra](https://raw.githubusercontent.com/advancedresearch/graph_solver/master/images/adinkra.png)
//!
//! ### Example: Cube
//!
//! To construct a cube using a graph solver
//! requires specifying how nodes connect to other nodes.
//! The information you give the solver is which colors these connections have,
//! but without telling which nodes in the graph should be connected.
//!
//! ![cube](https://raw.githubusercontent.com/advancedresearch/graph_solver/master/images/cube.png)
//!
//! ```rust̨
//! /*
//! === CUBE EXAMPLE ===
//!
//! Run with GraphViz (https://graphviz.org/):
//!
//!     cargo run --example cube | dot -Tpng > test.png
//!
//! */
//!
//! use graph_solver::*;
//!
//! // Notice that edges starts with `2`.
//! // This is because `0` is empty and `1` is no-edge.
//! const EDGE: Color = 2;
//!
//! fn main() {
//!     let mut g = Graph::new();
//!
//!     // Create a node pattern with 3 edges.
//!     let a = Node {
//!         color: 0,
//!         self_connected: false,
//!         edges: vec![Constraint {edge: EDGE, node: 0}; 3]
//!     };
//!
//!     // Add 8 vertices.
//!     for _ in 0..8 {g.push(a.clone())}
//!     g.no_triangles = true;
//!
//!     let solve_settings = SolveSettings::new();
//!     if let Some(solution) = g.solve(solve_settings) {
//!         println!("{}", solution.puzzle.graphviz(
//!             "sfdp",
//!             &["black"],
//!             &["black"]
//!         ));
//!     } else {
//!         eprintln!("<no solution>");
//!     }
//! }
//! ```
//!
//! ### Introduction to Graph Constraint Solving
//!
//! Each node has a color and a list of edge constraints.
//! The edge constraint stores an edge color and a target color for the adjacent node.
//!
//! This technique creates a powerful language to describe graphs compactly.
//! For example, all nodes that are locally similar,
//! can use a common description.
//!
//! Any graphs can be determined using
//! sufficient local information about the nodes and edges.
//! To do this, one can assign each node and edge a unique number.
//!
//! Therefore, to describe a graph in more detail,
//! one can simply add more colors!
//!
//! ### Design
//!
//! Uses the [quickbacktrack](https://crates.io/crates/quickbacktrack) library
//! for constraint solving.
//!
//! This allows starting with a partially solved graph,
//! override solving strategies, debug, or comparing the solution vs the original.
//!
//! - Node and edge colors are represents as `u64`
//! - A node color can be any value
//! - Edge colors usually start with `2`
//! - An edge color `0` is means no choice (neither empty or colored).
//! - An edge color `1` is means empty

pub use quickbacktrack::*;

/// The type of color.
pub type Color = u64;

/// Edges with value 0 are treated as empty.
pub const EMPTY_EDGE: Color = 0;
/// Edges with value 1 are treated as diconnected.
pub const DISCONNECTED_EDGE: Color = 1;

/// Stores information about graph.
///
/// An edge value `0` means no edge.
#[derive(Clone, Debug)]
pub struct Graph {
    /// Nodes.
    pub nodes: Vec<Node>,
    /// Edges.
    pub edges: Vec<Vec<Color>>,
    /// Pair constraints, using indices.
    pub pairs: Vec<(usize, usize)>,
    /// Whether triangle cycles are allowed.
    pub no_triangles: bool,
    /// Whether any shortest cycle for any vertex must be 4 or less.
    pub meet_quad: bool,
    /// Whether any node can be reached from any other node.
    pub connected: bool,
    /// Whether commutativity/anticommutativity is enabled for quads.
    ///
    /// When a quad commutes, the edges along one dimension have same colors.
    /// When a quad anticommutes, the edges along one dimension have same colors,
    /// but with an odd number of positive and negative signs (1+3 or 3+1).
    ///
    /// It is assumed that even and odd colors for edges
    /// above `2` anticommutes, e.g. `2` and `3` anticommutes.
    ///
    /// - When set to `Some(true)`, every quad commutes.
    /// - When set to `Some(false)`, every quad anticommutes.
    /// - When set to `None`
    pub commute_quad: Option<bool>,
    cache_has_triangles: std::cell::Cell<bool>,
    cache_connected: std::cell::Cell<bool>,
    cache_upper_triangle_disconnected: std::cell::Cell<bool>,
    cache_commute_quad_satisfied: std::cell::Cell<bool>,
    cache_node_satisfied: Vec<std::cell::Cell<bool>>,
}

impl Puzzle for Graph {
    type Pos = (usize, usize);
    type Val = Color;
    fn set(&mut self, (i, j): (usize, usize), val: Color) {
        let old = if j <= i {self.edges[i][j]} else {self.edges[j][i]};
        if j <= i {self.edges[i][j] = val} else {self.edges[j][i] = val}
        if old != 0 && val < 2 {
            self.cache_connected.set(false);
            self.cache_upper_triangle_disconnected.set(false);
        }
        if !(old == 0 && val == 1) {
            self.cache_commute_quad_satisfied.set(false);
        }
        if old != 0 {
            self.cache_has_triangles.set(false);
            self.cache_node_satisfied[i].set(false);
            self.cache_node_satisfied[j].set(false);
        }
    }
    fn get(&self, (i, j): (usize, usize)) -> Color {
        if j <= i {self.edges[i][j]} else {self.edges[j][i]}
    }
    fn print(&self) {
        for i in 0..self.nodes.len() {
            eprint!("{} ", self.nodes[i].color);
        }
        eprintln!("\n========================================");
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                eprint!("{} ", self.get((i, j)));
            }
            eprintln!("");
        }
    }
    fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, mut f: F) {
        let n = self.nodes.len();
        for i in 0..n {
            for j in i+1..n {
                let colors = self.colors((i, j));
                if colors.len() == 1 {
                    f(self, (i, j), colors[0]);
                }
            }
        }
    }
    fn is_solved(&self) -> bool {
        self.all_satisfied() &&
        self.pairs_satisfied() &&
        if self.no_triangles {!self.has_triangles()} else {true} &&
        if self.connected {self.is_connected()} else {true} &&
        if let Some(val) = self.commute_quad {self.commute_quad_satisfied(val)} else {true} &&
        if self.meet_quad {self.meet_quad_satisfied()} else {true}
    }
    fn remove(&mut self, other: &Graph) {
        let n = self.nodes.len();
        for i in 0..n {
            for j in i..n {
                if other.get((i, j)) != 0 {
                    self.set((i, j), 0);
                }
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Graph {Graph::new()}
}

impl Graph {
    /// Creates a new graph.
    ///
    /// Initialized with these default settings:
    /// - no-triangles: false
    /// - meet-quad: false
    /// - connected: false
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
            pairs: vec![],
            no_triangles: false,
            meet_quad: false,
            connected: false,
            commute_quad: None,
            cache_has_triangles: std::cell::Cell::new(false),
            cache_connected: std::cell::Cell::new(false),
            cache_upper_triangle_disconnected: std::cell::Cell::new(false),
            cache_commute_quad_satisfied: std::cell::Cell::new(false),
            cache_node_satisfied: vec![],
        }
    }

    /// Generates a GraphViz dot format.
    pub fn graphviz(&self, layout: &str, node_colors: &[&str], edge_colors: &[&str]) -> String {
        use std::fmt::Write;

        let mut s = String::new();
        writeln!(&mut s, "strict graph {{").unwrap();
        writeln!(&mut s, "  layout={}; edge[penwidth=4]", layout).unwrap();
        for i in 0..self.nodes.len() {
            writeln!(&mut s, "  {}[regular=true,style=filled,fillcolor={}];", i,
                   node_colors[self.nodes[i].color as usize % node_colors.len()]).unwrap();
        }
        for i in 0..self.nodes.len() {
            for (j, &ed) in self.edges[i].iter().enumerate() {
                if ed < 2 {continue};
                writeln!(&mut s, "  {} -- {}[color={}];", i, j,
                edge_colors[(ed - 2) as usize % edge_colors.len()]).unwrap();
            }
        }
        writeln!(&mut s, "}}").unwrap();
        s
    }

    /// Finds the first empty edge.
    pub fn fst_empty(&self) -> Option<(usize, usize)> {
        let n = self.nodes.len();
        for i in 0..n {
            for j in i..n {
                let s = self.colors((i, j)).len();
                if s == 0 {continue};
                if self.get((i, j)) == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// Finds the edge with the least possible colors.
    pub fn min_colors(&self) -> Option<(usize, usize)> {
        let mut min: Option<(usize, usize, usize)> = None;
        let n = self.nodes.len();
        'outer: for i in 0..n {
            for j in i..n {
                let s = self.colors((i, j)).len();
                if s == 0 {continue};
                if min.is_none() || min.unwrap().2 > s {
                    min = Some((i, j, s));
                    if s == 1 {break 'outer}
                }
            }
        }
        min.map(|n| (n.0, n.1))
    }

    /// Solves the graph puzzle using default strategy.
    ///
    /// The default strategy is `Graph::min_colors, Graph::colors`.
    pub fn solve(self, solve_settings: SolveSettings) -> Option<Solution<Graph>> {
        let solver = BackTrackSolver::new(self, solve_settings);
        solver.solve(
            Graph::min_colors,
            Graph::colors
        )
    }

    /// Adds a node description.
    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
        self.edges.push(vec![0; self.nodes.len()]);
        self.cache_node_satisfied.push(std::cell::Cell::new(false));
    }

    /// Adds a pair constraint.
    pub fn push_pair(&mut self, (i, j): (usize, usize)) {
        self.pairs.push((i.min(j), i.max(j)));
    }

    /// Returns a list of edge constraints that makes a node unsatisfied.
    ///
    /// If the returned list is empty, then the node is satisfied.
    pub fn node_satisfied(&self, i: usize) -> Vec<Constraint> {
        if self.cache_node_satisfied[i].get() {return vec![]};
        let mut res = vec![];
        let mut m = vec![false; self.nodes[i].edges.len()];
        for j in 0..self.nodes.len() {
            let edge = self.get((i, j));
            if edge == 0 {continue};
            for k in 0..m.len() {
                if m[k] {continue};
                let con = &self.nodes[i].edges[k];
                if con.edge == edge &&
                   con.node == self.nodes[j].color
                {
                    m[k] = true;
                    break;
                }
            }
        }
        for k in 0..m.len() {
            if !m[k] {
                res.push(self.nodes[i].edges[k].clone());
            }
        }
        if res.len() == 0 {
            self.cache_node_satisfied[i].set(true);
        }
        res
    }

    /// Returns `true` if all nodes are satisfied.
    pub fn all_satisfied(&self) -> bool {
        for i in 0..self.nodes.len() {
            if self.node_satisfied(i).len() != 0 {return false}
        }
        true
    }

    /// Returns `true` if all pair constraints are satisfied.
    pub fn pairs_satisfied(&self) -> bool {
        for &(i, j) in &self.pairs {
            if self.edges[j][i] < 2 {return false}
        }
        true
    }

    /// Returns whether the graph contains triangles.
    pub fn has_triangles(&self) -> bool {
        if self.cache_has_triangles.get() {return true};
        let n = self.nodes.len();
        for i in 0..n {
            for j in i+1..n {
                if self.get((i, j)) < 2 {continue};
                for k in j+1..n {
                    if self.get((j, k)) >= 2 &&
                       self.get((i, k)) >= 2
                    {
                        self.cache_has_triangles.set(true);
                        return true
                    }
                }
            }
        }
        false
    }

    /// Returns `true` when for any node,
    /// the greatest shortest cycle is either 3 or 4.
    pub fn meet_quad_satisfied(&self) -> bool {
        let n = self.nodes.len();
        for i in 0..n {
            let mut found = false;
            'outer: for j in 0..n {
                if i == j {continue};
                if self.get((i, j)) < 2 {continue};
                for k in j+1..n {
                    if k == i {continue};
                    if self.get((j, k)) < 2 &&
                       self.get((i, k)) < 2 {continue};
                    if self.get((j, k)) >= 2 &&
                       self.get((i, k)) >= 2 {
                        // Triangle.
                        found = true;
                        break 'outer;
                    }
                    for k2 in 0..n {
                        if k2 == i || k2 == j || k2 == k {continue};
                        if self.get((k, k2)) >= 2 &&
                           (
                            self.get((j, k)) >= 2 &&
                            self.get((i, k2)) >= 2 ||
                            self.get((i, k)) >= 2 &&
                            self.get((j, k2)) >= 2
                           )
                        {
                            found = true;
                            break 'outer;
                        }
                    }
                }
            }

            if !found {
                return false
            }
        }
        true
    }

    /// Returns `true` when for any quad,
    /// the commute property is satisfied.
    ///
    /// For more information, see `Graph::commute`.
    pub fn commute_quad_satisfied(&self, commute: bool) -> bool {
        if self.cache_commute_quad_satisfied.get() {return true};
        let n = self.nodes.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {continue};
                if self.get((i, j)) < 2 {continue};
                for k in j+1..n {
                    if k == i {continue};
                    if self.get((j, k)) < 2 &&
                       self.get((i, k)) < 2 {continue};
                    for k2 in 0..n {
                        if k2 == i || k2 == j || k2 == k {continue};
                        if self.get((k, k2)) >= 2 &&
                           self.get((j, k)) >= 2 &&
                           self.get((i, k2)) >= 2
                        {
                            let s = if commute {
                                self.get((i, j)) == self.get((k, k2)) &&
                                self.get((i, k2)) == self.get((j, k))
                            } else {
                                let ij = self.get((i, j));
                                let jk = self.get((j, k));
                                let kk2 = self.get((k, k2));
                                let ik2 = self.get((i, k2));
                                let x0 = (ij ^ 1) == kk2;
                                let x1 = ij == kk2;
                                let y0 = (jk ^ 1) == ik2;
                                let y1 = jk == ik2;
                                if (x0 ^ x1) && (y0 ^ y1) {x0 ^ y0} else {false}
                            };
                            if !s {return false}
                        } else if self.get((k, k2)) >= 2 &&
                                  self.get((i, k)) >= 2 &&
                                  self.get((j, k2)) >= 2
                        {
                            let s = if commute {
                                self.get((i, k)) == self.get((j, k2)) &&
                                self.get((i, j)) == self.get((k, k2))
                            } else {
                                let ik = self.get((i, k));
                                let ij = self.get((i, j));
                                let jk2 = self.get((j, k2));
                                let kk2 = self.get((k, k2));
                                let x0 = (ik ^ 1) == jk2;
                                let x1 = ik == jk2;
                                let y0 = (ij ^ 1) == kk2;
                                let y1 = ij == kk2;
                                if (x0 ^ x1) && (y0 ^ y1) {x0 ^ y0} else {false}
                            };
                            if !s {return false}
                        }
                    }
                }
            }
        }
        self.cache_commute_quad_satisfied.set(true);
        true
    }

    /// Returns `true` if all nodes can be reached from any node.
    pub fn is_connected(&self) -> bool {
        if self.cache_connected.get() {return true};
        let n = self.nodes.len();
        let mut reachable = vec![false; n];
        for i in 0..n {
            if self.get((0, i)) >= 2 {
                reachable[i] = true;
            }
        }
        loop {
            let mut changed = false;
            for i in 0..n {
                if !reachable[i] {
                    for j in 0..n {
                        if reachable[j] && self.get((i, j)) >= 2 {
                            reachable[i] = true;
                            changed = true;
                            break;
                        }
                    }
                }
            }
            if !changed {break}
        }

        let val = reachable.iter().all(|&b| b);
        if val {self.cache_connected.set(true)};
        val
    }

    /// Returns `true` if no-edges covers the upper right rectangle of the matrix form.
    ///
    /// This means that the graph will be disconnected.
    pub fn is_upper_right_disconnected(&self) -> bool {
        if self.cache_upper_triangle_disconnected.get() {return true};
        let n = self.nodes.len();
        if n % 2 != 0 {return false}
        for i in 0..n/2 {
            for j in n/2..n {
                if i == j {continue}
                if self.get((i, j)) != 1 {return false}
            }
        }
        self.cache_upper_triangle_disconnected.set(true);
        true
    }

    /// Returns a list of possible actions for a node.
    pub fn colors(&self, (i, j): (usize, usize)) -> Vec<Color> {
        if self.get((i, j)) != 0 {return vec![]};
        if !self.nodes[i].self_connected && i == j {return vec![]};
        if self.no_triangles && self.has_triangles() {return vec![]};
        if self.connected && self.is_upper_right_disconnected() {return vec![]};
        if let Some(val) = self.commute_quad {if !self.commute_quad_satisfied(val) {return vec![]}};
        let mut res = vec![];
        let errors = self.node_satisfied(i);
        let other_errors = self.node_satisfied(j);
        for err in &errors {
            if err.node != self.nodes[j].color {continue}
            for other_err in &other_errors {
                if err.edge == other_err.edge &&
                   other_err.node == self.nodes[i].color
                {
                    res.push(err.edge);
                    break;
                }
            }
        }
        res.push(1);
        res.sort();
        res.dedup();
        res
    }
}

/// Stores edge constraint.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    /// The edge color.
    pub edge: Color,
    /// The node color.
    pub node: Color,
}

/// Stores a description of a node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// The color of the node.
    pub color: Color,
    /// Whether the node can be self-connected.
    pub self_connected: bool,
    /// The edges constraints of the node.
    pub edges: Vec<Constraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple1() {
        let mut g = Graph::new();
        let a = Node {
            color: 1,
            self_connected: false,
            edges: vec![Constraint {edge: 2, node: 1}],
        };
        assert_eq!(g.nodes.len(), 0);
        g.push(a.clone());
        assert_eq!(g.node_satisfied(0), vec![
            Constraint {edge: 2, node: 1}
        ]);
        g.push(a.clone());
        assert_eq!(g.node_satisfied(0), vec![
            Constraint {edge: 2, node: 1}
        ]);
        assert_eq!(g.node_satisfied(1), vec![
            Constraint {edge: 2, node: 1}
        ]);
        assert_eq!(g.colors((0, 1)), vec![1, 2]);
        g.set((0, 1), 2);
        assert_eq!(g.node_satisfied(0), vec![]);
        g.set((0, 1), 2);
        assert!(g.all_satisfied());
    }
}
