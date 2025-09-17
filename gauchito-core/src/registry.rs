use crate::operations::*;
use std::collections::HashMap;

pub struct OperationRegistry {
    factories: HashMap<String, Box<dyn Fn(&str) -> Result<Box<dyn Operation>, String>>>,
}

impl OperationRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };

        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        // ==== CURSOR MOVEMENT OPERATIONS ====
        self.register("move_left", |_| Ok(Box::new(MoveLeft)));
        self.register("move_right", |_| Ok(Box::new(MoveRight)));
        self.register("move_up", |_| Ok(Box::new(MoveUp)));
        self.register("move_down", |_| Ok(Box::new(MoveDown)));
        self.register("move_line_start", |_| Ok(Box::new(MoveLineStart)));
        self.register("move_line_end", |_| Ok(Box::new(MoveLineEnd)));
        self.register("move_word_forward", |_| Ok(Box::new(MoveWordForward)));
        self.register("move_word_backward", |_| Ok(Box::new(MoveWordBackward)));
        self.register("move_big_word_forward", |_| {
            Ok(Box::new(MoveBigWordForward))
        });
        self.register("move_big_word_backward", |_| {
            Ok(Box::new(MoveBigWordBackward))
        });
        self.register("move_document_start", |_| Ok(Box::new(MoveDocumentStart)));
        self.register("move_document_end", |_| Ok(Box::new(MoveDocumentEnd)));
        self.register("move_matching_bracket", |_| {
            Ok(Box::new(MoveMatchingBracket))
        });
        self.register("move_paragraph_forward", |_| {
            Ok(Box::new(MoveParagraphForward))
        });
        self.register("move_paragraph_backward", |_| {
            Ok(Box::new(MoveParagraphBackward))
        });

        // ==== SELECTION OPERATIONS ====
        self.register("select_left", |_| Ok(Box::new(SelectLeft)));
        self.register("select_right", |_| Ok(Box::new(SelectRight)));
        self.register("select_up", |_| Ok(Box::new(SelectUp)));
        self.register("select_down", |_| Ok(Box::new(SelectDown)));
        self.register("select_word", |_| Ok(Box::new(SelectWord)));
        self.register("select_line", |_| Ok(Box::new(SelectLine)));
        self.register("select_all", |_| Ok(Box::new(SelectAll)));
        self.register("select_line_start", |_| Ok(Box::new(SelectLineStart)));
        self.register("select_line_end", |_| Ok(Box::new(SelectLineEnd)));
        self.register("clear_selection", |_| Ok(Box::new(ClearSelection)));

        // ==== TEXT INSERTION AND MODIFICATION ====
        self.register("insert_char", |params| match params {
            Some(p) if p.chars().count() == 1 => {
                Ok(Box::new(InsertChar::new(p.chars().next().unwrap())))
            }
            _ => Err("insert_char requires exactly one character".to_string()),
        });
        self.register("insert_string", |params| match params {
            Some(p) if !p.is_empty() => Ok(Box::new(InsertString::new(p.to_string()))),
            _ => Err("insert_string requires text parameter".to_string()),
        });
        self.register("insert_newline", |_| Ok(Box::new(InsertNewline)));
        self.register("insert_tab", |_| Ok(Box::new(InsertTab)));
        self.register("insert_spaces", |params| {
            let count = params.and_then(|p| p.parse().ok()).unwrap_or(1);
            Ok(Box::new(InsertSpaces::new(count)))
        });

        // ==== DELETION OPERATIONS ====
        self.register("delete_char", |_| Ok(Box::new(DeleteChar)));
        self.register("backspace", |_| Ok(Box::new(Backspace)));
        self.register("delete_word", |_| Ok(Box::new(DeleteWord)));
        self.register("delete_word_backward", |_| Ok(Box::new(DeleteWordBackward)));
        self.register("delete_line", |_| Ok(Box::new(DeleteLine)));
        self.register("delete_to_line_start", |_| Ok(Box::new(DeleteToLineStart)));
        self.register("delete_to_line_end", |_| Ok(Box::new(DeleteToLineEnd)));

        // ==== CLIPBOARD OPERATIONS ====
        self.register("copy", |_| Ok(Box::new(Copy)));
        self.register("cut", |_| Ok(Box::new(Cut)));
        self.register("paste", |params| {
            let text = params.unwrap_or("").to_string();
            Ok(Box::new(Paste::new(text)))
        });

        // ==== TEXT TRANSFORMATION OPERATIONS ====
        self.register("uppercase_selection", |_| Ok(Box::new(UppercaseSelection)));
        self.register("lowercase_selection", |_| Ok(Box::new(LowercaseSelection)));
        self.register("toggle_case_selection", |_| {
            Ok(Box::new(ToggleCaseSelection))
        });
        self.register("indent_selection", |_| Ok(Box::new(IndentSelection)));
        self.register("unindent_selection", |_| Ok(Box::new(UnindentSelection)));

        // ==== LINE OPERATIONS ====
        self.register("duplicate_line", |_| Ok(Box::new(DuplicateLine)));
        self.register("move_line_up", |_| Ok(Box::new(MoveLineUp)));
        self.register("move_line_down", |_| Ok(Box::new(MoveLineDown)));
        self.register("insert_line_above", |_| Ok(Box::new(InsertLineAbove)));
        self.register("insert_line_below", |_| Ok(Box::new(InsertLineBelow)));

        // ==== SEARCH AND REPLACE OPERATIONS ====
        self.register("find_next", |params| match params {
            Some(p) if !p.is_empty() => Ok(Box::new(FindNext::new(p.to_string()))),
            _ => Err("find_next requires search pattern".to_string()),
        });
        self.register("find_previous", |params| match params {
            Some(p) if !p.is_empty() => Ok(Box::new(FindPrevious::new(p.to_string()))),
            _ => Err("find_previous requires search pattern".to_string()),
        });
        self.register("replace", |params| {
            if let Some(p) = params {
                if let Some((pattern, replacement)) = p.split_once(" with ") {
                    Ok(Box::new(Replace::new(
                        pattern.to_string(),
                        replacement.to_string(),
                    )))
                } else {
                    Err("replace requires format: 'pattern with replacement'".to_string())
                }
            } else {
                Err("replace requires pattern and replacement".to_string())
            }
        });
        self.register("replace_all", |params| {
            if let Some(p) = params {
                if let Some((pattern, replacement)) = p.split_once(" with ") {
                    Ok(Box::new(ReplaceAll::new(
                        pattern.to_string(),
                        replacement.to_string(),
                    )))
                } else {
                    Err("replace_all requires format: 'pattern with replacement'".to_string())
                }
            } else {
                Err("replace_all requires pattern and replacement".to_string())
            }
        });

        // ==== HISTORY OPERATIONS ====
        self.register("undo", |_| Ok(Box::new(Undo)));
        self.register("redo", |_| Ok(Box::new(Redo)));

        // ==== MODE AND SYSTEM OPERATIONS ====
        self.register("switch_mode", |params| match params {
            Some(p) if !p.is_empty() => Ok(Box::new(SwitchMode::new(p.to_string()))),
            _ => Err("switch_mode requires a target mode parameter".to_string()),
        });
        self.register("exit", |_| Ok(Box::new(Exit)));
        self.register("save", |_| Ok(Box::new(Save)));
        self.register("save_as", |params| match params {
            Some(p) if !p.is_empty() => Ok(Box::new(SaveAs::new(std::path::PathBuf::from(p)))),
            _ => Err("save_as requires file path parameter".to_string()),
        });

        // ==== JUMP OPERATIONS ====
        self.register("jump_to_line", |params| {
            if let Some(p) = params {
                if let Ok(line_num) = p.parse::<usize>() {
                    Ok(Box::new(JumpToLine::new(line_num)))
                } else {
                    Err("jump_to_line requires valid line number".to_string())
                }
            } else {
                Err("jump_to_line requires line number parameter".to_string())
            }
        });
        self.register("jump_to_character", |params| {
            if let Some(p) = params {
                if let Ok(pos) = p.parse::<usize>() {
                    Ok(Box::new(JumpToCharacter::new(pos)))
                } else {
                    Err("jump_to_character requires valid character position".to_string())
                }
            } else {
                Err("jump_to_character requires position parameter".to_string())
            }
        });

        // ==== CONVENIENCE ALIASES ====
        self.register("h", |_| Ok(Box::new(MoveLeft)));
        self.register("j", |_| Ok(Box::new(MoveDown)));
        self.register("k", |_| Ok(Box::new(MoveUp)));
        self.register("l", |_| Ok(Box::new(MoveRight)));
        self.register("w", |_| Ok(Box::new(MoveWordForward)));
        self.register("b", |_| Ok(Box::new(MoveWordBackward)));
        self.register("W", |_| Ok(Box::new(MoveBigWordForward)));
        self.register("B", |_| Ok(Box::new(MoveBigWordBackward)));
        self.register("0", |_| Ok(Box::new(MoveLineStart)));
        self.register("$", |_| Ok(Box::new(MoveLineEnd)));
        self.register("gg", |_| Ok(Box::new(MoveDocumentStart)));
        self.register("G", |_| Ok(Box::new(MoveDocumentEnd)));
        self.register("x", |_| Ok(Box::new(DeleteChar)));
        self.register("X", |_| Ok(Box::new(Backspace)));
        self.register("dd", |_| Ok(Box::new(DeleteLine)));
        self.register("yy", |_| Ok(Box::new(Copy)));
        self.register("p", |params| {
            let text = params.unwrap_or("").to_string();
            Ok(Box::new(Paste::new(text)))
        });
        self.register("u", |_| Ok(Box::new(Undo)));
        self.register("r", |_| Ok(Box::new(Redo)));
        self.register("o", |_| Ok(Box::new(InsertLineBelow)));
        self.register("O", |_| Ok(Box::new(InsertLineAbove)));

        // Special commands
        self.register(":", |params| {
            if let Some(p) = params {
                match p {
                    "q" => Ok(Box::new(Exit)),
                    "w" => Ok(Box::new(Save)),
                    "wq" => {
                        // This would need to be handled differently in practice
                        // For now, just save
                        Ok(Box::new(Save))
                    }
                    _ if p.starts_with("w ") => {
                        let path = &p[2..];
                        Ok(Box::new(SaveAs::new(std::path::PathBuf::from(path))))
                    }
                    _ => Err(format!("Unknown command: :{}", p)),
                }
            } else {
                Err("Command mode requires parameter".to_string())
            }
        });
    }

    /// Registers an operation factory.
    /// `factory` is given an optional `&str` of params.
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn(Option<&str>) -> Result<Box<dyn Operation>, String> + 'static,
    {
        self.factories.insert(
            name.to_string(),
            Box::new(move |params| {
                if params.is_empty() {
                    factory(None)
                } else {
                    factory(Some(params))
                }
            }),
        );
    }

    /// Creates an operation from a name and optional parameters.
    pub fn create(&self, name: &str, params: &str) -> Result<Box<dyn Operation>, String> {
        match self.factories.get(name) {
            Some(factory) => factory(params),
            None => Err(format!("Unknown operation: {}", name)),
        }
    }

    /// Lists all registered operations
    pub fn list_operations(&self) -> Vec<String> {
        let mut ops: Vec<String> = self.factories.keys().cloned().collect();
        ops.sort();
        ops
    }

    /// Check if an operation exists
    pub fn has_operation(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

impl Default for OperationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
