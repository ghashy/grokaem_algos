use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

pub fn run_me() {
    let mut graph = Graph::new();

    let elem1 = MyType {
        field1: String::new(),
        field2: 0,
    };
    let elem2 = MyType {
        field1: String::new(),
        field2: 1,
    };
    let elem3 = MyType {
        field1: String::new(),
        field2: 2,
    };
    let elem4 = MyType {
        field1: String::new(),
        field2: 3,
    };
    let elem5 = MyType {
        field1: String::new(),
        field2: 5,
    };

    let node0 = graph.add(elem1);
    let node1 = graph.add(elem2);
    let node2 = graph.add(elem3);
    let node3 = graph.add(elem4);
    let node4 = graph.add(elem5);

    graph.create_ref(node0, node2).unwrap();
    graph.create_ref(node0, node4).unwrap();
    graph.create_ref(node2, node1).unwrap();
    graph.create_ref(node1, node4).unwrap();
    graph.create_ref(node4, node3).unwrap();
    graph.create_ref(node1, node3).unwrap();

    println!("Shortest way: {:?}", graph.find_shortest_way(0, 3));
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct MyType {
    field1: String,
    field2: i32,
}

#[derive(Debug, Error)]
enum GraphError<'a, T> {
    #[error("Vertex not found")]
    NoVertex(&'a T),
}

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<T>,
    edges: BTreeMap<usize, Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Route<T>(Vec<T>);

impl<T> Route<T> {
    fn new(origin: T) -> Self {
        Route(vec![origin])
    }
    fn add(&mut self, element: T) {
        self.0.push(element);
    }
    fn head(&self) -> &T {
        self.0.last().unwrap()
    }
}

impl<T: Debug + Eq + PartialEq + Hash> Graph<T> {
    fn new() -> Graph<T> {
        Graph {
            nodes: Vec::new(),
            edges: BTreeMap::new(),
        }
    }

    fn add(&mut self, elem: T) -> usize {
        self.nodes.push(elem);
        self.edges.insert(self.nodes.len() - 1, Vec::new()); // Use length directly
        self.nodes.len() - 1
    }

    fn create_ref(
        &mut self,
        from: usize,
        to: usize,
    ) -> Result<(), GraphError<T>> {
        let len = self.nodes.len();
        let is_to_in_graph = len > to;

        match self.edges.get_mut(&from) {
            Some(from_connections) => {
                if is_to_in_graph {
                    if !from_connections.contains(&to) {
                        from_connections.push(to);
                    }
                    Ok(())
                } else {
                    Err(GraphError::NoVertex(&self.nodes[to])) // Direct access with indexing
                }
            }
            None => Err(GraphError::NoVertex(&self.nodes[from])),
        }
    }

    fn find_shortest_way(
        &self,
        origin: usize,
        target: usize,
    ) -> Option<Route<usize>> {
        let mut checked = BTreeSet::new();
        let mut deque: VecDeque<Route<usize>> = VecDeque::new();

        match self.edges.get(&origin) {
            Some(inner) => {
                deque.extend(inner.iter().map(|idx| {
                    let mut route = Route::new(origin);
                    route.add(*idx);
                    route
                }));

                while let Some(cur) = deque.pop_front() {
                    if target.eq(cur.head()) {
                        return Some(cur);
                    } else if let Some(inner) = self.edges.get(&cur.head()) {
                        if !checked.contains(cur.head()) {
                            checked.insert(*cur.head());
                            deque.extend(inner.iter().map(|idx| {
                                let mut local = cur.clone();
                                local.add(*idx);
                                local
                            }));
                        }
                        continue;
                    }
                    continue;
                }
                return None;
            }
            None => return None,
        }
    }
}
