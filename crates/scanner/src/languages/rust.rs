use super::{ExtractedSymbol, LanguageScanner, SymbolType};
use sentinel_arc_core::error::{BrainError, BrainResult};
use tree_sitter::{Node as TsNode, Parser};

pub struct RustScanner;

impl RustScanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RustScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageScanner for RustScanner {
    fn scan(&self, content: &str) -> BrainResult<Vec<ExtractedSymbol>> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .map_err(|e| BrainError::validation(format!("Failed to set rust language: {e}")))?;

        let tree = parser
            .parse(content, None)
            .ok_or_else(|| BrainError::validation("Failed to parse rust file"))?;

        let root_node = tree.root_node();
        Ok(extract_from_node(root_node, content.as_bytes()))
    }
}

fn extract_from_node(node: TsNode, source: &[u8]) -> Vec<ExtractedSymbol> {
    let mut symbols = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();

        match kind {
            "mod_item" | "struct_item" | "enum_item" | "trait_item" => {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = name_node.utf8_text(source).unwrap_or("").to_string();
                    if !name.is_empty() {
                        let children = extract_from_node(child, source);
                        symbols.push(ExtractedSymbol {
                            name,
                            symbol_type: SymbolType::Module,
                            children,
                        });
                        continue;
                    }
                }
            }
            "impl_item" => {
                if let Some(type_node) = child.child_by_field_name("type") {
                    let name = type_node.utf8_text(source).unwrap_or("").to_string();
                    if !name.is_empty() {
                        let children = extract_from_node(child, source);
                        symbols.push(ExtractedSymbol {
                            name,
                            symbol_type: SymbolType::Module,
                            children,
                        });
                        continue;
                    }
                }
            }
            "function_item" => {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = name_node.utf8_text(source).unwrap_or("").to_string();
                    if !name.is_empty() {
                        symbols.push(ExtractedSymbol {
                            name,
                            symbol_type: SymbolType::Function,
                            children: vec![],
                        });
                        continue;
                    }
                }
            }
            _ => {}
        }

        // If we didn't handle it as a named container, recurse down to find nested items
        let mut inner_symbols = extract_from_node(child, source);
        symbols.append(&mut inner_symbols);
    }

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_scanner_extraction() {
        let code = r#"
            mod inner {
                struct MyStruct {
                    field: i32,
                }
                impl MyStruct {
                    fn do_something(&self) {}
                }
            }
            fn global_fn() {}
        "#;

        let scanner = RustScanner::new();
        let symbols = scanner.scan(code).unwrap();

        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "inner");
        assert_eq!(symbols[0].symbol_type, SymbolType::Module);

        assert_eq!(symbols[1].name, "global_fn");
        assert_eq!(symbols[1].symbol_type, SymbolType::Function);

        let inner_children = &symbols[0].children;
        assert_eq!(inner_children.len(), 2); // MyStruct, and impl MyStruct

        let mut child_names: Vec<String> = inner_children.iter().map(|c| c.name.clone()).collect();
        child_names.sort();
        assert_eq!(child_names, vec!["MyStruct", "MyStruct"]);

        let impl_block = inner_children
            .iter()
            .find(|c| !c.children.is_empty())
            .unwrap();
        assert_eq!(impl_block.children.len(), 1);
        assert_eq!(impl_block.children[0].name, "do_something");
        assert_eq!(impl_block.children[0].symbol_type, SymbolType::Function);
    }
}
