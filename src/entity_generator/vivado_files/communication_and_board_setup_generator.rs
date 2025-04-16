use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct CommunicationAndBoardSetup {}

impl CommunicationAndBoardSetup {
    pub(crate) fn new() -> CommunicationAndBoardSetup {
        CommunicationAndBoardSetup {}
    }
}

impl GenerateVhdlCode for CommunicationAndBoardSetup {
    fn template_name(&self) -> String {
        "communication_and_board_setup.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "communication_and_board_setup.h".to_string()
    }
}

#[cfg(test)]
mod communication_and_board_setup_file_tests_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let cfile = CommunicationAndBoardSetup::new();
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&cfile, &tera, &PathBuf::from("target/test_files"));
    }
}
