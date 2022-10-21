use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct EnginesField {
    node: Option<node_semver::Range>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PackageJson {
    engines: Option<EnginesField>,
}

impl PackageJson {
    pub fn node_range(&self) -> Option<&node_semver::Range> {
        self.engines
            .as_ref()
            .and_then(|engines| engines.node.as_ref())
    }
}
