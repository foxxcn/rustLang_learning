use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct DAG {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl DAG {
    pub fn new() -> Self {
        DAG {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) {
        self.nodes.insert(node.to_string());
        self.edges.entry(node.to_string()).or_insert_with(HashSet::new);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) -> Result<(), &'static str> {
        if !self.nodes.contains(from) || !self.nodes.contains(to) {
            return Err("Both nodes must exist in the graph.");
        }

        if self.detect_cycle(from, to) {
            return Err("Adding this edge would create a cycle.");
        }

        self.edges.get_mut(from).unwrap().insert(to.to_string());
        Ok(())
    }

    fn detect_cycle(&self, start: &str, end: &str) -> bool {
        let mut visited = HashSet::new();
        self.has_path(end, start, &mut visited)
    }

    fn has_path(&self, current: &str, target: &str, visited: &mut HashSet<String>) -> bool {
        if current == target {
            return true;
        }

        if visited.contains(current) {
            return false;
        }

        visited.insert(current.to_string());

        if let Some(neighbors) = self.edges.get(current) {
            for neighbor in neighbors {
                if self.has_path(neighbor, target, visited) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut dag = DAG::new();
        dag.add_node("A");
        assert!(dag.nodes.contains("A"));
    }

    #[test]
    fn test_add_edge() {
        let mut dag = DAG::new();
        dag.add_node("A");
        dag.add_node("B");
        assert!(dag.add_edge("A", "B").is_ok());
        assert!(dag.edges["A"].contains("B"));
    }

    #[test]
    fn test_add_edge_with_nonexistent_node() {
        let mut dag = DAG::new();
        dag.add_node("A");
        let result = dag.add_edge("A", "B");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_edge_creates_cycle() {
        let mut dag = DAG::new();
        dag.add_node("A");
        dag.add_node("B");
        dag.add_node("C");
        dag.add_edge("A", "B").unwrap();
        dag.add_edge("B", "C").unwrap();
        let result = dag.add_edge("C", "A");
        assert!(result.is_err());
    }

    #[test]
    fn test_has_path() {
        let mut dag = DAG::new();
        dag.add_node("A");
        dag.add_node("B");
        dag.add_node("C");
        dag.add_edge("A", "B").unwrap();
        dag.add_edge("B", "C").unwrap();
        let mut visited = HashSet::new();
        assert!(dag.has_path("A", "C", &mut visited));
        let mut visited = HashSet::new();
        assert!(!dag.has_path("C", "A", &mut visited));
    }
}
