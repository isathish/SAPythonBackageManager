use std::collections::{HashMap, HashSet};
use petgraph::{Graph, Directed};
use petgraph::dot::{Dot, Config};

// Dependency visualization
pub struct DependencyVisualizer;

impl DependencyVisualizer {
    pub fn create_dependency_graph(
        package_name: &str,
        dependencies: &HashMap<String, Vec<String>>,
        transitive: bool,
    ) -> Graph<String, (), Directed> {
        let mut graph = Graph::new();
        let mut node_indices = HashMap::new();

        // Add root node
        let root_idx = graph.add_node(package_name.to_string());
        node_indices.insert(package_name.to_string(), root_idx);

        if let Some(deps) = dependencies.get(package_name) {
            Self::add_dependencies_to_graph(
                &mut graph,
                &mut node_indices,
                package_name,
                deps,
                dependencies,
                transitive,
                &mut HashSet::new(),
            );
        }

        graph
    }

    fn add_dependencies_to_graph(
        graph: &mut Graph<String, (), Directed>,
        node_indices: &mut HashMap<String, petgraph::graph::NodeIndex>,
        parent: &str,
        deps: &[String],
        all_dependencies: &HashMap<String, Vec<String>>,
        transitive: bool,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(parent) {
            return; // Prevent cycles
        }
        visited.insert(parent.to_string());

        let parent_idx = *node_indices.get(parent).unwrap();

        for dep in deps {
            let dep_idx = if let Some(&idx) = node_indices.get(dep) {
                idx
            } else {
                let idx = graph.add_node(dep.clone());
                node_indices.insert(dep.clone(), idx);
                idx
            };

            graph.add_edge(parent_idx, dep_idx, ());

            if transitive {
                if let Some(sub_deps) = all_dependencies.get(dep) {
                    Self::add_dependencies_to_graph(
                        graph,
                        node_indices,
                        dep,
                        sub_deps,
                        all_dependencies,
                        transitive,
                        visited,
                    );
                }
            }
        }

        visited.remove(parent);
    }

    pub fn export_dot(graph: &Graph<String, (), Directed>) -> String {
        format!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]))
    }
}
