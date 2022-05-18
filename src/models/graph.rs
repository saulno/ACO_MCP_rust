use std::collections::{HashMap, HashSet};

use rand::Rng;

#[derive(Debug)]
pub struct Graph {
    pub adj_mtx: Vec<Vec<bool>>,
    pub edge_dict: HashMap<usize, HashSet<usize>>,
    pub n_vertex: usize
}

impl Graph {
    pub fn new(text: String) -> Graph {
        let mut graph = Graph { adj_mtx: vec![], edge_dict: HashMap::new(), n_vertex: 0 };

        let lines: Vec<String> = text.
            lines().
            map(|s| String::from(s)).
            collect();

        let sizes: Vec<usize> = lines[1].
            trim().
            split(" ").
            map(|s| s.parse().unwrap()).
            collect();
        graph.n_vertex = sizes[0];
        graph.adj_mtx = vec![vec![false; sizes[1]]; sizes[0]];
        for i in 0..sizes[0] {
            graph.edge_dict.insert(i, HashSet::new());
        }

        for edge in lines[2..].into_iter() {
            let node: Vec<usize> = edge.
                trim().
                split(" ").
                map(|s| s.parse().unwrap()).
                collect();
            
            let (v1, v2) = (node[0] - 1, node[1] - 1);
            graph.adj_mtx[v1][v2] = true;
            graph.adj_mtx[v2][v1] = true;
            if let Some(set) = graph.edge_dict.get_mut(&v1) {
                set.insert(v2);
            }
            if let Some(set) = graph.edge_dict.get_mut(&v2) {
                set.insert(v1);
            }

        }

        graph
    }

    pub fn is_edge(&self, vertex_1: usize, vertex_2: usize) -> bool {
        self.adj_mtx[vertex_1][vertex_2]
    }

    pub fn select_random_vertex(&self) -> usize {
        rand::thread_rng().gen_range(0..self.n_vertex)
    }

    pub fn get_neighbor_candidates(&self, vertex: usize) -> HashSet<usize> {
        if let Some(set) = self.edge_dict.get(&vertex) {
            set.clone()
        } else {
            HashSet::new()
        }
    }
}