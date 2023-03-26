use crate::graph::Graph;
use crate::view_api::*;
use rand::prelude::*;
use std::collections::HashSet;

/// This function is a graph generation model based upon:
/// Barabási, Albert-László, and Réka Albert. "Emergence of scaling in random networks." science 286.5439 (1999): 509-512.
///
/// Given a graph this function will add a user defined number of vertices, each with a user defined number of edges.
/// This is an iterative algorithm where at each `step` a vertex is added and its neighbours are chosen from the pool of nodes already within the network.
/// For this model the neighbours are chosen proportionally based upon their degree, favouring nodes with higher degree (more connections).
/// This sampling is conducted without replacement.
///
/// **Note:**  If the provided graph doesnt have enough nodes/edges for the initial sample, the min number of both will be added before generation begins.
///
/// # Arguments
/// * `graph` - The graph you wish to add vertices and edges to
/// * `vertices_to_add` - The amount of vertices you wish to add to the graph (steps)
/// * `edges_per_step` - The amount of edges a joining vertex should add to the graph
/// # Examples
///
/// ```
/// use docbrown_db::graph::Graph;
/// let graph = Graph::new(2);
//  ba_preferential_attachment(&graph, 1000, 10);
/// ```
pub fn ba_preferential_attachment(graph: &Graph, vertices_to_add: usize, edges_per_step: usize) {
    let mut rng = rand::thread_rng();
    let mut latest_time = match graph.latest_time() {
        None => 0,
        Some(time) => time,
    };
    let view = graph.window(i64::MIN, i64::MAX);
    let mut ids: Vec<u64> = view.vertices().id().collect();
    let mut degrees: Vec<usize> = view.vertices().map(|v| v.degree()).collect();
    let mut edge_count: usize = degrees.iter().sum();

    let mut max_id = match ids.iter().max() {
        Some(id) => *id,
        None => 0,
    };

    while ids.len() < edges_per_step {
        max_id += 1;
        graph.add_vertex(latest_time, max_id, &vec![]).map_err(|err| println!("{:?}", err)).ok();
        degrees.push(0);
        ids.push(max_id);
    }

    if graph.num_edges() < edges_per_step {
        for pos in 1..ids.len() {
            graph.add_edge(latest_time, ids[pos], ids[pos - 1], &vec![]);
            edge_count += 2;
            degrees[pos] += 1;
            degrees[pos - 1] += 1;
        }
    }

    for _ in 0..vertices_to_add {
        max_id += 1;
        latest_time += 1;
        let mut normalisation = edge_count.clone();
        let mut positions_to_skip: HashSet<usize> = HashSet::new();

        for _ in 0..edges_per_step {
            let mut sum = 0;
            let rand_num = rng.gen_range(1..=normalisation);
            for pos in 0..ids.len() {
                if !positions_to_skip.contains(&pos) {
                    sum += degrees[pos];
                    if sum >= rand_num {
                        positions_to_skip.insert(pos);
                        normalisation -= degrees[pos];
                        break;
                    }
                }
            }
        }
        for pos in positions_to_skip {
            let dst = ids[pos];
            degrees[pos] += 1;
            graph.add_edge(latest_time, max_id, dst, &vec![]);
        }
        ids.push(max_id);
        degrees.push(edges_per_step.clone());
        edge_count += edges_per_step * 2;
    }
}

//TODO need to benchmark the creation of these networks
#[cfg(test)]
mod preferential_attachment_tests {
    use super::*;
    use crate::graphgen::random_attachment::random_attachment;
    #[test]
    fn blank_graph() {
        let graph = Graph::new(2);
        ba_preferential_attachment(&graph, 1000, 10);
        assert_eq!(graph.num_edges(), 10009);
        assert_eq!(graph.num_vertices(), 1010);
    }

    #[test]
    fn only_nodes() {
        let graph = Graph::new(2);
        for i in 0..10 {
            graph.add_vertex(i, i as u64, &vec![]).map_err(|err| println!("{:?}", err)).ok();
        }

        ba_preferential_attachment(&graph, 1000, 5);
        assert_eq!(graph.num_edges(), 5009);
        assert_eq!(graph.num_vertices(), 1010);
    }

    #[test]
    fn prior_graph() {
        let graph = Graph::new(2);
        random_attachment(&graph, 1000, 3);
        ba_preferential_attachment(&graph, 500, 4);
        assert_eq!(graph.num_edges(), 5000);
        assert_eq!(graph.num_vertices(), 1503);
    }
}
