pub mod rust;

use sentinel_arc_core::error::BrainResult;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolType {
    Module,
    Function,
}

#[derive(Debug)]
pub struct ExtractedSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub children: Vec<ExtractedSymbol>,
}

pub trait LanguageScanner: Send + Sync {
    /// Scans the provided source code and extracts a hierarchical list of symbols.
    fn scan(&self, content: &str) -> BrainResult<Vec<ExtractedSymbol>>;
}

/// Returns the appropriate LanguageScanner for the given file path, if supported.
pub fn get_scanner_for_file(path: &Path) -> Option<Box<dyn LanguageScanner>> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" => Some(Box::new(rust::RustScanner::new())),
        _ => None,
    }
}
