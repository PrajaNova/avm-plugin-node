mod aliases;
mod install;
mod versions;

use avm_plugin_api::{tool_dir, ToolProvider, ToolVersion, ToolVersionQuery};
use std::path::PathBuf;

pub use aliases::aliases_from_package_json;

#[derive(Debug)]
pub struct NodeProvider;

impl NodeProvider {
    pub fn bin_path_for(&self, version: &str, binary: &str) -> anyhow::Result<Option<PathBuf>> {
        let candidate = tool_dir("node")?.join(version).join("bin").join(binary);
        Ok(candidate.exists().then_some(candidate))
    }
}

impl ToolProvider for NodeProvider {
    fn name(&self) -> &str {
        "node"
    }

    fn is_installed(&self, version: &str) -> bool {
        self.bin_path_for(version, "node").ok().flatten().is_some()
    }

    fn installed_versions(&self) -> anyhow::Result<Vec<String>> {
        avm_plugin_api::list_installed("node", |v| self.is_installed(v))
    }

    fn available_versions(&self, query: ToolVersionQuery) -> anyhow::Result<Vec<ToolVersion>> {
        versions::available_versions(query)
    }

    fn executable_path(&self, version: &str) -> anyhow::Result<Option<PathBuf>> {
        self.bin_path_for(version, "node")
    }

    fn install(&self, version: &str) -> anyhow::Result<()> {
        install::install_node(version)
    }

    fn uninstall(&self, version: &str) -> anyhow::Result<()> {
        avm_plugin_api::remove_version("node", version)
    }
}
