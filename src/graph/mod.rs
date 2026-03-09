use std::collections::VecDeque;

use crate::math::Arithmetic;

pub trait NodeTrait {
    type ValueType;

    fn new(index: usize, val: Self::ValueType) -> Self;

    fn get(&self) -> &Self::ValueType;
    fn get_mut(&mut self) -> &mut Self::ValueType;

    fn add_child(&mut self, child: &mut Self);
    fn remove_child(&mut self, child: &mut Self);

    fn children(&self) -> Vec<usize>;
    fn parent(&self) -> Option<usize>;
    fn set_parent(&mut self, par: &Self) -> Result<(), String>;
}

pub trait GraphTrait {
    type ValueType;
    type NodeType: NodeTrait<ValueType = Self::ValueType>;
    type EdgeType: Arithmetic;
    const IS_DIRECTED: bool;

    fn new() -> Self;
    fn with_capacity(n: usize) -> Self;
    fn get_unoccupied_index(&self) -> usize;
    fn _insert_node(&mut self, node: Self::NodeType);

    fn node_count(&self) -> usize;

    fn edge_count(&self) -> usize;

    fn add_node(&mut self, val: Self::ValueType) -> Result<usize, String> {
        let new_idx = self.get_unoccupied_index();
        let new_node = Self::NodeType::new(new_idx, val);
        self._insert_node(new_node);
        Ok(new_idx)
    }

    fn get_node(&self, idx: usize) -> Option<&Self::NodeType>;
    fn get_node_mut(&mut self, idx: usize) -> Option<&mut Self::NodeType>;

    fn connect_edge(&mut self, idx1: usize, idx2: usize) -> Result<(), String>;
    fn disconnect_edge(&mut self, idx1: usize, idx2: usize) -> Result<(), String>;

    fn add_child(&mut self, idx: usize, val: Self::ValueType) -> Result<usize, String> {
        let new_idx = self.add_node(val)?;
        self.connect_edge(idx, new_idx)?;
        Ok(new_idx)
    }

    fn remove_child(&mut self, idx: usize, ch: usize) -> Result<(), String> {
        self.disconnect_edge(idx, ch)
    }

    fn children(&self, idx:usize) -> Vec<&Self::NodeType>{
        match self.get_node(idx){
            Some(n) => {
                n.children().iter().map(|i| self.get_node(*i).unwrap()).collect::<Vec<_>>()
            },
            _ => vec![]
        }
    }
}

pub struct Node<T> {
    idx: usize,
    val: T,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl<T> NodeTrait for Node<T> {
    type ValueType = T;

    fn new(index: usize, val: Self::ValueType) -> Self {
        Node {
            idx: index,
            val,
            children: Vec::new(),
            parent: None,
        }
    }

    fn get(&self) -> &Self::ValueType {
        &self.val
    }

    fn get_mut(&mut self) -> &mut Self::ValueType {
        &mut self.val
    }

    fn add_child(&mut self, child: &mut Self) {
        self.children.push(child.idx);
    }

    fn remove_child(&mut self, child: &mut Self) {
        self.children.remove(child.idx);
    }

    fn children(&self) -> Vec<usize> {
        self.children.clone()
    }

    fn parent(&self) -> Option<usize> {
        self.parent
    }

    fn set_parent(&mut self, parent: &Node<T>) -> Result<(), String> {
        self.parent = Some(parent.idx);
        Ok(())
    }
}

pub struct UnDirectedGraph<V> {
    nodes: Vec<Node<V>>,
    last_idx: usize,
    e_count: usize,
}

impl<V> GraphTrait for UnDirectedGraph<V> {
    type EdgeType = u32;
    type ValueType = V;
    type NodeType = Node<V>;

    const IS_DIRECTED: bool = false;

    fn new() -> Self {
        UnDirectedGraph {
            nodes: Vec::new(),
            last_idx: 0,
            e_count: 0,
        }
    }

    fn with_capacity(n: usize) -> Self {
        UnDirectedGraph {
            nodes: Vec::with_capacity(n),
            last_idx: 0,
            e_count: 0,
        }
    }

    fn get_unoccupied_index(&self) -> usize {
        self.last_idx
    }

    fn _insert_node(&mut self, node: Node<V>) {
        self.nodes.push(node);
        self.last_idx += 1;
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.e_count
    }

    fn get_node(&self, idx: usize) -> Option<&Node<V>> {
        self.nodes.get(idx)
    }

    fn get_node_mut(&mut self, idx: usize) -> Option<&mut Self::NodeType> {
        self.nodes.get_mut(idx)
    }

    fn connect_edge(&mut self, idx1: usize, idx2: usize) -> Result<(), String> {
        let n1_ptr: *mut Node<V> = self.get_node_mut(idx1).ok_or("No valid index")? as *mut _;
        let n2 = self.get_node_mut(idx2).ok_or("No valid index")?;

        let n1 = unsafe { &mut *n1_ptr };
        n1.add_child(n2);

        Ok(())
    }

    fn disconnect_edge(&mut self, idx1: usize, idx2: usize) -> Result<(), String> {
        let n1_ptr: *mut Node<V> = self.get_node_mut(idx1).ok_or("No valid index")? as *mut _;
        let n2 = self.get_node_mut(idx2).ok_or("No valid index")?;

        let n1 = unsafe { &mut *n1_ptr };
        n1.remove_child(n2);

        Ok(())
    }
}

// BFS

pub struct BFS<'a, G: GraphTrait> {
    g: &'a G,
    visited: Vec<bool>,
    que: VecDeque<usize>,
}

impl<'a, G: GraphTrait> BFS<'a, G> {
    pub fn new(g: &'a G, node: usize) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut que = VecDeque::with_capacity(n);
        visited[node] = true;
        que.push_back(node);
        BFS { g, visited, que }
    }
}

impl<'a, G: GraphTrait> Iterator for BFS<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.que.pop_front() {
            let curr_node = self.g.get_node(curr).unwrap();
            for idx in curr_node.children() {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.que.push_back(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}

pub struct IndexBFS<'a, G: GraphTrait> {
    g: &'a G,
    visited: Vec<bool>,
    que: VecDeque<usize>,
    direction: bool,
}

impl<'a, G: GraphTrait> IndexBFS<'a, G> {
    pub fn new(g: &'a G, node: usize, direction: bool) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut que = VecDeque::with_capacity(n);
        visited[node] = true;
        que.push_back(node);
        IndexBFS {
            g,
            visited,
            que,
            direction,
        }
    }
}

impl<'a, G: GraphTrait> Iterator for IndexBFS<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.que.pop_front() {
            let curr_node = self.g.get_node(curr).unwrap();
            let mut children = curr_node.children();
            if self.direction {
                children.sort_unstable();
            } else {
                children.sort_unstable_by(|a, b| b.cmp(a));
            }
            for idx in curr_node.children() {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.que.push_back(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}

pub struct OrderingBFS<'a, G: GraphTrait, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    g: &'a G,
    visited: Vec<bool>,
    que: VecDeque<usize>,
    ordering_fn: F,
}

impl<'a, G: GraphTrait, F> OrderingBFS<'a, G, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    pub fn new(g: &'a G, node: usize, ordering_fn: F) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut que = VecDeque::with_capacity(n);
        visited[node] = true;
        que.push_back(node);
        OrderingBFS {
            g,
            visited,
            que,
            ordering_fn,
        }
    }
}

impl<'a, G: GraphTrait, F> Iterator for OrderingBFS<'a, G, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.que.pop_front() {
            let curr_node = self.g.get_node(curr).unwrap();
            let mut children = curr_node.children();
            children.sort_unstable_by(|a, b| {
                (self.ordering_fn)(
                    self.g.get_node(*a).unwrap().get(),
                    self.g.get_node(*b).unwrap().get(),
                )
            });
            for idx in children {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.que.push_back(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}

// DFS

pub struct DFS<'a, G: GraphTrait> {
    g: &'a G,
    visited: Vec<bool>,
    stk: Vec<usize>,
}

impl<'a, G: GraphTrait> DFS<'a, G> {
    pub fn new(g: &'a G, node: usize) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut stk = Vec::with_capacity(n);
        visited[node] = true;
        stk.push(node);
        DFS { g, visited, stk }
    }
}

impl<'a, G: GraphTrait> Iterator for DFS<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.stk.pop() {
            let curr_node = self.g.get_node(curr).unwrap();
            for idx in curr_node.children() {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.stk.push(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}

pub struct IndexDFS<'a, G: GraphTrait> {
    g: &'a G,
    visited: Vec<bool>,
    stk: Vec<usize>,
    direction: bool,
}

impl<'a, G: GraphTrait> IndexDFS<'a, G> {
    pub fn new(g: &'a G, node: usize, direction: bool) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut stk = Vec::with_capacity(n);
        visited[node] = true;
        stk.push(node);
        IndexDFS {
            g,
            visited,
            stk,
            direction,
        }
    }
}


impl<'a, G: GraphTrait> Iterator for IndexDFS<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.stk.pop() {
            let curr_node = self.g.get_node(curr).unwrap();
            let mut children = curr_node.children();
            if self.direction {
                children.sort_unstable();
            } else {
                children.sort_unstable_by(|a, b| b.cmp(a));
            }
            for idx in curr_node.children() {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.stk.push(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}


pub struct OrderingDFS<'a, G: GraphTrait, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    g: &'a G,
    visited: Vec<bool>,
    stk: Vec<usize>,
    ordering_fn: F,
}

impl<'a, G: GraphTrait, F> OrderingDFS<'a, G, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    pub fn new(g: &'a G, node: usize, ordering_fn: F) -> Self {
        let n = g.node_count();
        let mut visited = vec![false; n];
        let mut stk = Vec::with_capacity(n);
        visited[node] = true;
        stk.push(node);
        OrderingDFS {
            g,
            visited,
            stk,
            ordering_fn,
        }
    }
}

impl<'a, G: GraphTrait, F> Iterator for OrderingDFS<'a, G, F>
where
    F: FnMut(&G::ValueType, &G::ValueType) -> std::cmp::Ordering,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.stk.pop() {
            let curr_node = self.g.get_node(curr).unwrap();
            let mut children = curr_node.children();
            children.sort_unstable_by(|a, b| {
                (self.ordering_fn)(
                    self.g.get_node(*a).unwrap().get(),
                    self.g.get_node(*b).unwrap().get(),
                )
            });
            for idx in children {
                if !self.visited[idx] {
                    self.visited[idx] = true;
                    self.stk.push(idx);
                }
            }
            Some(curr)
        } else {
            None
        }
    }
}
