use std::cmp::Ordering;
use std::collections::BinaryHeap;

// 边的结构体
#[derive(Debug, Clone, Copy)]
struct Edge {
    node1: usize,
    node2: usize,
    weight: usize,
}

// 实现边的比较，用于优先队列
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.weight.cmp(&self.weight)) // 反转顺序，使得BinaryHeap成为最小堆
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

// 并查集结构体
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    fn union(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);

        if root_u != root_v {
            if self.rank[root_u] > self.rank[root_v] {
                self.parent[root_v] = root_u;
            } else if self.rank[root_u] < self.rank[root_v] {
                self.parent[root_u] = root_v;
            } else {
                self.parent[root_v] = root_u;
                self.rank[root_u] += 1;
            }
        }
    }
}

// Kruskal最小生成树算法
fn kruskal_mst(num_nodes: usize, edges: Vec<Edge>) -> Vec<Edge> {
    let mut uf = UnionFind::new(num_nodes);
    let mut mst = Vec::new();
    let mut edge_heap = BinaryHeap::from(edges);

    while let Some(edge) = edge_heap.pop() {
        if uf.find(edge.node1) != uf.find(edge.node2) {
            uf.union(edge.node1, edge.node2);
            mst.push(edge);
        }
    }

    mst
}

// 测试函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal_mst() {
        let edges = vec![
            Edge {
                node1: 0,
                node2: 1,
                weight: 10,
            },
            Edge {
                node1: 0,
                node2: 2,
                weight: 6,
            },
            Edge {
                node1: 0,
                node2: 3,
                weight: 5,
            },
            Edge {
                node1: 1,
                node2: 3,
                weight: 15,
            },
            Edge {
                node1: 2,
                node2: 3,
                weight: 4,
            },
        ];

        let mst = kruskal_mst(4, edges);
        let total_weight: usize = mst.iter().map(|edge| edge.weight).sum();
        assert_eq!(total_weight, 19);

        let expected_mst = vec![
            Edge {
                node1: 2,
                node2: 3,
                weight: 4,
            },
            Edge {
                node1: 0,
                node2: 3,
                weight: 5,
            },
            Edge {
                node1: 0,
                node2: 1,
                weight: 10,
            },
        ];

        assert_eq!(mst.len(), expected_mst.len());

        for edge in expected_mst {
            assert!(mst.contains(&edge));
        }
    }
}

fn main() {
    let edges = vec![
        Edge {
            node1: 0,
            node2: 1,
            weight: 10,
        },
        Edge {
            node1: 0,
            node2: 2,
            weight: 6,
        },
        Edge {
            node1: 0,
            node2: 3,
            weight: 5,
        },
        Edge {
            node1: 1,
            node2: 3,
            weight: 15,
        },
        Edge {
            node1: 2,
            node2: 3,
            weight: 4,
        },
    ];

    let mst = kruskal_mst(4, edges);
    for edge in mst {
        println!(
            "Edge from {} to {} with weight {}",
            edge.node1, edge.node2, edge.weight
        );
    }
}
