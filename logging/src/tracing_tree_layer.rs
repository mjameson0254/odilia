use tracing_tree::HierarchicalLayer;
pub fn init_layer() -> HierarchicalLayer {
    HierarchicalLayer::new(4)
        .with_targets(true)
        .with_bracketed_fields(true)
}
