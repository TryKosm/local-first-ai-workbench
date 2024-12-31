use local_first_ai_workbench::tools::builtin_tools;

#[test]
fn has_shell_tool() {
    assert!(builtin_tools().contains(&"shell"));
}
