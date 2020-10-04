use super::shell::Shell;
use super::{Call, ExpectCommandOutput};

pub(crate) fn test_node_version<S: Shell>(
    expected_version: &'static str,
) -> ExpectCommandOutput<S, Call> {
    ExpectCommandOutput::new(
        Call::new("node", vec!["-v"]),
        expected_version,
        "Node version",
    )
}
