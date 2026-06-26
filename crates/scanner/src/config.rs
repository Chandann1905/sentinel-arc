use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// The root directory of the workspace to scan.
    pub workspace_root: PathBuf,
    /// Maximum file size to scan in bytes. Files larger than this will be ignored.
    pub max_file_size_bytes: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            workspace_root: PathBuf::from("."),
            max_file_size_bytes: 2 * 1024 * 1024, // 2MB
        }
    }
}

impl ScanConfig {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            ..Default::default()
        }
    }
}
