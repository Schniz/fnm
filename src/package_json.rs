use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
struct EnginesField {
    node: Option<node_semver::Range>,
}

#[derive(Debug, Deserialize, Default)]
struct DevEnginesField {
    runtime: Option<RuntimeField>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RuntimeField {
    Single(DevEngine),
    Multiple(Vec<DevEngine>),
}

#[derive(Debug, Deserialize)]
struct DevEngine {
    name: String,
    version: Option<node_semver::Range>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PackageJson {
    engines: Option<EnginesField>,
    #[serde(rename = "devEngines")]
    dev_engines: Option<DevEnginesField>,
}

impl PackageJson {
    pub fn node_range(&self) -> Option<&node_semver::Range> {
        self.engines
            .as_ref()
            .and_then(|engines| engines.node.as_ref())
    }

    pub fn dev_node_range(&self) -> Option<&node_semver::Range> {
        self.dev_engines
            .as_ref()
            .and_then(|dev_engines| dev_engines.runtime.as_ref())
            .and_then(|runtime| {
                let engines = match runtime {
                    RuntimeField::Single(engine) => std::slice::from_ref(engine),
                    RuntimeField::Multiple(engines) => engines.as_slice(),
                };
                engines
                    .iter()
                    .find(|engine| engine.name.to_lowercase() == "node")
                    .and_then(|engine| engine.version.as_ref())
            })
    }
}
