use anyhow::{anyhow, Context, Result};
use avm_plugin_api::{run_timed, tool_dir};
use std::fs;
use std::path::Path;
use std::process::Command;

const TAR_TIMEOUT_MS: u64 = 300_000;
const CURL_OUTER_TIMEOUT_MS: u64 = 180_000;

pub fn install_node(version: &str) -> Result<()> {
    let version = version.trim_start_matches('v');
    let platform = platform_name()?;
    let tools_root = tool_dir("node")?;
    let target = tools_root.join(version);
    if target.join("bin").join("node").exists() {
        return Ok(());
    }

    // Download + extract inside tools_root: the half-done dirs never count as
    // installed (no `<version>/bin/node`), and the final rename stays on one fs.
    let archive_name = format!("node-v{version}-{platform}.tar.gz");
    let tmp_root = tools_root.join(format!(".tmp-{version}"));
    let archive_path = tmp_root.join(&archive_name);
    let extract_path = tmp_root.join(format!("node-v{version}-{platform}"));

    if tmp_root.exists() {
        fs::remove_dir_all(&tmp_root).context("failed to clean previous node install temp dir")?;
    }
    fs::create_dir_all(&tmp_root).context("failed to create node install temp dir")?;
    fetch_archive(version, &archive_name, &archive_path)?;

    let mut tar = Command::new("tar");
    tar.arg("-xzf").arg(&archive_path).arg("-C").arg(&tmp_root);
    run_timed(tar, TAR_TIMEOUT_MS, "node archive extraction", "AVM_TAR_TIMEOUT")?;

    if target.exists() {
        fs::remove_dir_all(&target).context("failed to replace existing node install")?;
    }
    fs::rename(&extract_path, &target).context("failed to move node install into place")?;
    let _ = fs::remove_dir_all(&tmp_root);
    Ok(())
}

fn fetch_archive(version: &str, archive_name: &str, destination: &Path) -> Result<()> {
    let mirror = std::env::var("AVM_NODE_DIST_URL").unwrap_or_else(|_| "https://nodejs.org/dist".to_string());
    let local = Path::new(&mirror).join(format!("v{version}")).join(archive_name);
    if local.exists() {
        fs::copy(&local, destination)
            .with_context(|| format!("failed to copy node archive from {}", local.display()))?;
        return Ok(());
    }

    let url = format!("{}/v{version}/{archive_name}", mirror.trim_end_matches('/'));
    let mut curl = Command::new("curl");
    curl.args(["-fL", "--connect-timeout", "10", "--max-time", "120", &url, "-o"])
        .arg(destination);
    run_timed(curl, CURL_OUTER_TIMEOUT_MS, "node download", "AVM_CURL_TIMEOUT")
        .with_context(|| format!("failed to download Node.js from {url}"))
}

fn platform_name() -> Result<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => Ok("darwin-arm64"),
        ("macos", "x86_64") => Ok("darwin-x64"),
        ("linux", "aarch64") => Ok("linux-arm64"),
        ("linux", "x86_64") => Ok("linux-x64"),
        (os, arch) => Err(anyhow!("unsupported Node.js platform: {os}-{arch}")),
    }
}
