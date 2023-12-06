use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use convenient_skiplist::SkipList;

pub trait HasId {
    fn id(&self) -> String;
}

#[derive(Clone)]
#[allow(dead_code)]
struct PlannedVisitation {
    node_id: String,
    priority: i64,
}

impl PartialEq<Self> for PlannedVisitation {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for PlannedVisitation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

#[allow(dead_code)]
struct Graph<T: HasId> {
    nodes: HashMap<String, Rc<T>>,
    children: HashMap<String, HashSet<String>>,
    visitation_queue: SkipList<PlannedVisitation>,
    priority_generator: i64,
}

#[allow(dead_code)]
impl<T: HasId> Graph<T> {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            children: HashMap::new(),
            visitation_queue: SkipList::new(),
            priority_generator: 0,
        }
    }

    fn add(&mut self, node: T, with_it: &mut dyn FnMut(&mut Graph<T>, &Weak<T>)) {
        let rc = Rc::new(node);
        let weak = Rc::downgrade(&rc);
        self.children.insert(rc.id(), HashSet::new());
        self.nodes.insert(rc.id(), rc);
        with_it(self, &weak);
    }

    fn add_child(&mut self, parent: &Weak<T>, child: &Weak<T>) {
        let children = self.children.entry(parent.upgrade().unwrap().id()).or_insert(HashSet::new());
        children.insert(child.upgrade().unwrap().id());
    }

    fn with_node(&mut self, id: &String, visitor: &mut dyn FnMut(&mut Graph<T>, &Weak<T>)) {
        visitor(self, &Rc::downgrade(self.nodes.get(id).unwrap()));
    }

    fn with_children(&mut self, parent: &Weak<T>, visitor: &mut dyn FnMut(&mut Graph<T>, &Weak<T>)) {
        if let Some(children) = self.children.get(&parent.upgrade().unwrap().id()) {
            let cloned_children = children.clone();
            for child in cloned_children.iter() {
                visitor(self, &Rc::downgrade(self.nodes.get(child).unwrap()));
            }
        }
    }

    fn enqueue_visitation(&mut self, node: &Weak<T>) {
        self.priority_generator += 1;
        self.queue_visitation_with_priority(node, self.priority_generator);
    }

    fn queue_visitation_with_priority(&mut self, node: &Weak<T>, priority: i64) {
        if priority > self.priority_generator {
            self.priority_generator = priority;
        }
        self.priority_generator += self.priority_generator;
        self.visitation_queue.insert(PlannedVisitation {
            node_id: node.upgrade().unwrap().id(),
            priority,
        });
    }

    fn visit_until_done(&mut self, visitor: &mut dyn FnMut(&mut Graph<T>, Weak<T>)) {
        while let Some(PlannedVisitation { node_id, .. }) = self.visitation_queue.pop_front() {
            visitor(self, Rc::downgrade(self.nodes.get(&node_id).unwrap()));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Graph, HasId};

    struct TestNode {
        id: String,
    }

    impl HasId for TestNode {
        fn id(&self) -> String {
            self.id.clone()
        }
    }

    #[test]
    fn exploration() {
        let mut graph: Graph<TestNode> = Graph::new();

        graph.add(TestNode { id: "a".to_string() }, &mut |graph_a, a| {
            graph_a.add(TestNode { id: "b".to_string() }, &mut |graph_b, b| {
                graph_b.add_child(a, b);
            });
        });

        graph.with_node(&"a".to_string(), &mut |graph_a, a| {
            assert_eq!(a.upgrade().unwrap().id(), "a");

            graph_a.queue_visitation_with_priority(a, 10);

            graph_a.with_children(a, &mut |graph_b, b| {
                assert_eq!(b.upgrade().unwrap().id(), "b");

                graph_b.queue_visitation_with_priority(b, 1);
            });
        });

        let mut visited: Vec<String> = Vec::new();

        graph.visit_until_done(&mut |graph_v, node| {
            let id = node.upgrade().unwrap().id();
            if !visited.contains(&id) {
                graph_v.enqueue_visitation(&node);
            }
            visited.push(id.clone());
            println!("Visiting {}", id);
        });

        assert_eq!(visited, vec!["b", "a", "b", "a"]);
    }
}