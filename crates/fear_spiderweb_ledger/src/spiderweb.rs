use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use crate::deed::DeedEvent;

pub type FearWeb = DiGraph<DeedEvent, f32>; // edge weight = FEAR impact

pub struct SpiderwebAnalyzer {
    pub web: FearWeb,
    pub node_map: HashMap<Uuid, NodeIndex>,
}

impl SpiderwebAnalyzer {
    pub fn new() -> Self {
        Self { web: DiGraph::new(), node_map: HashMap::new() }
    }

    pub fn add_deed(&mut self, deed: DeedEvent) -> NodeIndex {
        let idx = self.web.add_node(deed.clone());
        self.node_map.insert(deed.event_id, idx);
        // Add edges to prior events (direct/indirect logic)
        // ... (windowed temporal + predicate correlation)
        idx
    }

    // Root cause analysis: reverse traversal from overloaded nodes
    pub fn find_root_causes(&self, start: NodeIndex, max_depth: usize) -> Vec<Vec<NodeIndex>> {
        // DFS/BFS reverse with decay weighting
        vec![] // implement path collection with FEAR/DECAY thresholds
    }

    // Generate literature Markdown
    pub fn generate_documentation(&self) -> String {
        let mut doc = String::from("# Church-of-FEAR Spiderweb of FEAR Documentation\n\n");
        doc.push_str("## Interconnected Causes: Birds, Spiders, Bees\n");
        doc.push_str("Spiders: vibration detection → FEAR as learning signal (extended cognition).\n");
        doc.push_str("Bees: collective recovery corridors & pollination of good deeds.\n");
        doc.push_str("Birds: song of freedom propagating CALMSTABLE zones.\n\n");
        // Add graph stats, stable zones, eco_grant recommendations
        doc
    }

    // Export DOT for visualization (Graphviz) or plotters image
    pub fn export_dot(&self) -> String { /* ... */ "digraph FearWeb { ... }".to_string() }
}
