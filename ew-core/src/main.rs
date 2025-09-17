use ew_core::context::Context;
use ew_core::operations::OperationResult;
use ew_core::registry::OperationRegistry;

fn main() {
    let mut context = Context::with_content("Hello World!\nThis is a test.\nLast line here.");
    let registry = OperationRegistry::new();

    println!("=== Text Editor Operations Demo ===");
    println!("Initial content:");
    print_buffer_state(&context);

    // Example 1: Basic cursor movement
    println!("\n1. Moving cursor right 5 times:");
    for _ in 0..5 {
        execute_operation(&registry, &mut context, "move_right", "");
    }
    print_buffer_state(&context);

    // Example 2: Select a word
    println!("\n2. Selecting current word:");
    execute_operation(&registry, &mut context, "select_word", "");
    print_buffer_state(&context);

    // Example 3: Replace selection with new text
    println!("\n3. Replacing selection with 'Goodbye':");
    execute_operation(&registry, &mut context, "insert_string", "Goodbye");
    print_buffer_state(&context);

    // Example 4: Move to next line and duplicate it
    println!("\n4. Moving to next line and duplicating it:");
    execute_operation(&registry, &mut context, "move_down", "");
    execute_operation(&registry, &mut context, "duplicate_line", "");
    print_buffer_state(&context);

    // Example 5: Select entire line and make it uppercase
    println!("\n5. Selecting line and making it uppercase:");
    execute_operation(&registry, &mut context, "select_line", "");
    execute_operation(&registry, &mut context, "uppercase_selection", "");
    print_buffer_state(&context);

    // Example 6: Undo the last operation
    println!("\n6. Undoing last operation:");
    execute_operation(&registry, &mut context, "undo", "");
    print_buffer_state(&context);

    // Example 7: Jump to specific line
    println!("\n7. Jumping to line 3:");
    execute_operation(&registry, &mut context, "jump_to_line", "3");
    print_buffer_state(&context);

    // Example 8: Insert line below and add text
    println!("\n8. Inserting new line below and adding text:");
    execute_operation(&registry, &mut context, "insert_line_below", "");
    execute_operation(&registry, &mut context, "insert_string", "New line added!");
    print_buffer_state(&context);

    // Example 9: Find and replace
    println!("\n9. Finding 'test' and replacing with 'demo':");
    execute_operation(&registry, &mut context, "find_next", "test");
    execute_operation(&registry, &mut context, "replace", "test with demo");
    print_buffer_state(&context);

    // Example 10: Show vim-like commands
    println!("\n10. Using vim-like shortcuts:");
    execute_operation(&registry, &mut context, "gg", ""); // Go to start
    execute_operation(&registry, &mut context, "w", ""); // Move word forward
    execute_operation(&registry, &mut context, "w", ""); // Move word forward
    print_buffer_state(&context);

    // Example 11: List all available operations
    println!("\n11. Available operations:");
    let operations = registry.list_operations();
    println!("Total operations: {}", operations.len());

    // Group operations by category for better display
    let mut movement_ops = Vec::new();
    let mut edit_ops = Vec::new();
    let mut selection_ops = Vec::new();
    let mut other_ops = Vec::new();

    for op in operations {
        if op.starts_with("move_")
            || ["h", "j", "k", "l", "w", "b", "W", "B", "0", "$", "gg", "G"].contains(&op.as_str())
        {
            movement_ops.push(op);
        } else if op.starts_with("select_") {
            selection_ops.push(op);
        } else if op.starts_with("insert_")
            || op.starts_with("delete_")
            || op.contains("case")
            || op.contains("indent")
        {
            edit_ops.push(op);
        } else {
            other_ops.push(op);
        }
    }

    println!("Movement operations ({}):", movement_ops.len());
    for op in movement_ops.chunks(5) {
        println!("  {}", op.join(", "));
    }

    println!("Selection operations ({}):", selection_ops.len());
    for op in selection_ops.chunks(5) {
        println!("  {}", op.join(", "));
    }

    println!("Editing operations ({}):", edit_ops.len());
    for op in edit_ops.chunks(5) {
        println!("  {}", op.join(", "));
    }

    println!("Other operations ({}):", other_ops.len());
    for op in other_ops.chunks(5) {
        println!("  {}", op.join(", "));
    }

    // Show final buffer stats
    let stats = context.buffer_stats();
    println!("\nFinal buffer statistics:");
    println!("  Lines: {}", stats.total_lines);
    println!("  Characters: {}", stats.total_chars);
    println!(
        "  Current position: Line {}, Column {}",
        stats.current_line, stats.current_column
    );
    println!("  Selection: {} characters", stats.selected_chars);
    println!("  Modified: {}", stats.is_modified);
}

fn execute_operation(
    registry: &OperationRegistry,
    context: &mut Context,
    name: &str,
    params: &str,
) {
    match registry.create(name, params) {
        Ok(operation) => {
            match operation.execute(context) {
                OperationResult::Continue => {
                    // Operation completed successfully
                }
                OperationResult::SwitchMode(mode) => {
                    println!("  -> Switched to mode: {}", mode);
                }
                OperationResult::Exit => {
                    println!("  -> Exit requested");
                }
            }
        }
        Err(e) => {
            println!("  -> Error: {}", e);
        }
    }
}

fn print_buffer_state(context: &Context) {
    let content_str = context.buffer().content().to_string();
    let (start, end) = context.selection().range();

    // Build a visual representation of the buffer with the selection
    let mut display = String::new();

    // Get the part before the selection
    if let Some(pre) = content_str.get(..start) {
        display.push_str(pre);
    }

    // Add the selection marker(s)
    if start == end {
        // It's a cursor
        display.push('|');
    } else {
        // It's a selection range
        display.push('[');
        if let Some(mid) = content_str.get(start..end) {
            display.push_str(mid);
        }
        display.push(']');
    }

    // Add the part after the selection
    if let Some(post) = content_str.get(end..) {
        display.push_str(post);
    }

    println!("Content: {}", display.replace('\n', "\\n"));
    println!(
        "Position: Line {}, Column {} (char {})",
        context.current_line(),
        context.current_column(),
        context.selection().head
    );

    let stats = context.buffer_stats();
    if stats.selected_chars > 0 {
        println!("Selection: {} characters", stats.selected_chars);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut context = Context::with_content("Hello World!");
        let registry = OperationRegistry::new();

        // Test cursor movement
        let op = registry.create("move_right", "").unwrap();
        op.execute(&mut context);
        assert_eq!(context.selection().head, 1);

        // Test character insertion
        let op = registry.create("insert_char", "X").unwrap();
        op.execute(&mut context);
        assert_eq!(context.buffer().content().to_string(), "HXello World!");

        // Test undo
        let op = registry.create("undo", "").unwrap();
        op.execute(&mut context);
        assert_eq!(context.buffer().content().to_string(), "Hello World!");
    }

    #[test]
    fn test_word_operations() {
        let mut context = Context::with_content("Hello beautiful world!");
        let registry = OperationRegistry::new();

        // Move to word boundary
        let op = registry.create("move_word_forward", "").unwrap();
        op.execute(&mut context);
        assert_eq!(context.selection().head, 5); // Should be at space after "Hello"

        // Select word
        let op = registry.create("select_word", "").unwrap();
        op.execute(&mut context);
        let (start, end) = context.selection().range();
        let selected = context.buffer().content().slice(start..end).to_string();
        assert!(selected.contains("beautiful") || selected.trim().contains("beautiful"));
    }

    #[test]
    fn test_line_operations() {
        let mut context = Context::with_content("Line 1\nLine 2\nLine 3");
        let registry = OperationRegistry::new();

        // Jump to line 2
        let op = registry.create("jump_to_line", "2").unwrap();
        op.execute(&mut context);
        assert_eq!(context.current_line(), 2);

        // Duplicate line
        let original_content = context.buffer().content().to_string();
        let op = registry.create("duplicate_line", "").unwrap();
        op.execute(&mut context);
        let new_content = context.buffer().content().to_string();
        assert_ne!(original_content, new_content);
        assert!(new_content.lines().count() > original_content.lines().count());
    }

    #[test]
    fn test_selection_operations() {
        let mut context = Context::with_content("Hello World\nSecond Line");
        let registry = OperationRegistry::new();

        // Select all
        let op = registry.create("select_all", "").unwrap();
        op.execute(&mut context);
        let (start, end) = context.selection().range();
        assert_eq!(start, 0);
        assert_eq!(end, context.buffer().len_chars());

        // Clear selection
        let op = registry.create("clear_selection", "").unwrap();
        op.execute(&mut context);
        assert!(context.selection().is_cursor());
    }

    #[test]
    fn test_text_transformation() {
        let mut context = Context::with_content("hello world");
        let registry = OperationRegistry::new();

        // Select all text
        let op = registry.create("select_all", "").unwrap();
        op.execute(&mut context);

        // Make uppercase
        let op = registry.create("uppercase_selection", "").unwrap();
        op.execute(&mut context);
        assert_eq!(context.buffer().content().to_string(), "HELLO WORLD");

        // Undo
        let op = registry.create("undo", "").unwrap();
        op.execute(&mut context);
        assert_eq!(context.buffer().content().to_string(), "hello world");
    }

    #[test]
    fn test_search_operations() {
        let mut context = Context::with_content("The quick brown fox jumps over the lazy dog");
        let registry = OperationRegistry::new();

        // Find "fox"
        let op = registry.create("find_next", "fox").unwrap();
        op.execute(&mut context);
        let (start, end) = context.selection().range();
        let selected = context.buffer().content().slice(start..end).to_string();
        assert_eq!(selected, "fox");

        // Replace with "cat"
        let op = registry.create("replace", "fox with cat").unwrap();
        op.execute(&mut context);
        assert!(context.buffer().content().to_string().contains("cat"));
        assert!(!context.buffer().content().to_string().contains("fox"));
    }

    #[test]
    fn test_vim_shortcuts() {
        let mut context = Context::with_content("Hello World");
        let registry = OperationRegistry::new();

        // Test vim-like movement
        let op = registry.create("w", "").unwrap(); // move_word_forward
        op.execute(&mut context);
        assert_ne!(context.selection().head, 0);

        let op = registry.create("0", "").unwrap(); // move_line_start
        op.execute(&mut context);
        assert_eq!(context.selection().head, 0);

        let op = registry.create("$", "").unwrap(); // move_line_end
        op.execute(&mut context);
        assert_eq!(context.selection().head, "Hello World".len());
    }

    #[test]
    fn test_registry_operations() {
        let registry = OperationRegistry::new();

        // Test that we have a good number of operations
        let operations = registry.list_operations();
        assert!(operations.len() > 50); // We should have many operations

        // Test that key operations exist
        assert!(registry.has_operation("move_left"));
        assert!(registry.has_operation("move_right"));
        assert!(registry.has_operation("insert_char"));
        assert!(registry.has_operation("delete_char"));
        assert!(registry.has_operation("undo"));
        assert!(registry.has_operation("redo"));
        assert!(registry.has_operation("select_all"));
        assert!(registry.has_operation("find_next"));

        // Test vim shortcuts
        assert!(registry.has_operation("h"));
        assert!(registry.has_operation("j"));
        assert!(registry.has_operation("k"));
        assert!(registry.has_operation("l"));
    }
}
