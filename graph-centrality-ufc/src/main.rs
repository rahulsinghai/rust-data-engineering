// This program models a network of fighters and their fights using an undirected graph.
// Each fighter is represented as a node, and each fight between two fighters is represented as an edge.
// The program calculates and prints the closeness centrality for each fighter, which is a measure of how central a fighter is within the network.

// The petgraph crate provides graph data structures and algorithms.
// The graph module contains the UnGraph type, which represents an undirected graph.
// The NodeIndex type is used to identify nodes in the graph.
use petgraph::graph::{NodeIndex, UnGraph};
// The Direction enum is used to specify the direction of edges in the graph.
use petgraph::Direction;
use std::fmt;

// Represents a fighter with a name.
#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/

impl Fighter {
    // Creates a new Fighter with the given name.
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

// Here, we implement the Display trait for the Fighter struct to specify how a Fighter should be displayed.
impl fmt::Display for Fighter {
    // Formats the Fighter's name for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Adds an edge between two nodes in the graph, representing a fight between two fighters.
// Parameters:
// - graph: A mutable reference to the graph.
// - nodes: A slice of node indices corresponding to the fighters. &[NodeIndex] is a slice of NodeIndex.
// - a: The index of the first fighter in the nodes slice. usize is an unsigned integer type.
// - b: The index of the second fighter in the nodes slice.
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

// - Creates a graph and adds nodes for each fighter.
// - Adds edges between nodes to represent fights between fighters.
// - Calculates and prints the closeness centrality for each fighter.
// - Provides an explanation of the centrality values for specific fighters.
fn main() {
    let mut graph = UnGraph::new_undirected(); // Creates an undirected graph.

    // An array of `Fighter` instances representing UFC fighters.
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    // A vector of `NodeIndex` instances representing the nodes in the graph.
    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))  // Adds a node to the graph for each fighter. add_node returns a NodeIndex.
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32; // Counts the number of outgoing edges from the node.
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        // Explanation
        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");
    }
}
