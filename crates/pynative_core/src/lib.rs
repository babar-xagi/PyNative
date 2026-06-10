use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeInfo {
    pub name: &'static str,
    pub phase: &'static str,
    pub rust_core: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetNode {
    pub kind: String,
    #[serde(default)]
    pub props: Value,
    #[serde(default)]
    pub children: Vec<WidgetNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetTreeSummary {
    pub root: String,
    pub node_count: usize,
    pub max_depth: usize,
}

pub fn runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        name: "PyNative UI",
        phase: "phase-1",
        rust_core: "pynative_core",
    }
}

pub fn widget_tree_from_json(input: &str) -> Result<WidgetNode, serde_json::Error> {
    serde_json::from_str(input)
}

pub fn summarize_widget_tree_json(input: &str) -> Result<WidgetTreeSummary, serde_json::Error> {
    let root = widget_tree_from_json(input)?;
    Ok(summarize_widget_tree(&root))
}

pub fn summarize_widget_tree(root: &WidgetNode) -> WidgetTreeSummary {
    WidgetTreeSummary {
        root: root.kind.clone(),
        node_count: count_nodes(root),
        max_depth: max_depth(root),
    }
}

fn count_nodes(node: &WidgetNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn max_depth(node: &WidgetNode) -> usize {
    1 + node.children.iter().map(max_depth).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_tree_shape() {
        let tree = WidgetNode {
            kind: "Window".to_string(),
            props: Value::Null,
            children: vec![WidgetNode {
                kind: "Text".to_string(),
                props: Value::Null,
                children: vec![],
            }],
        };

        let summary = summarize_widget_tree(&tree);

        assert_eq!(summary.root, "Window");
        assert_eq!(summary.node_count, 2);
        assert_eq!(summary.max_depth, 2);
    }
}
