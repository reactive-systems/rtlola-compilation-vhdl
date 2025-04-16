use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct MainCFileGenerator {}

impl MainCFileGenerator {
    pub(crate) fn new() -> MainCFileGenerator {
        MainCFileGenerator {}
    }
}

impl GenerateVhdlCode for MainCFileGenerator {
    fn template_name(&self) -> String {
        "main_cfile.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "main.c".to_string()
    }
}

#[cfg(test)]
mod main_cfile_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let cfile = MainCFileGenerator::new();
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&cfile, &tera, &PathBuf::from("target/test_files"));
    }
}
