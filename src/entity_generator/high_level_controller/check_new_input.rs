use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct CheckNewInput {}

impl CheckNewInput {
    pub(crate) fn new() -> CheckNewInput {
        CheckNewInput {}
    }
}

impl GenerateVhdlCode for CheckNewInput {
    fn template_name(&self) -> String {
        "check_new_input.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "check_new_input.vhdl".to_string()
    }
}

#[cfg(test)]
mod check_new_input_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let pack = CheckNewInput::new();
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
