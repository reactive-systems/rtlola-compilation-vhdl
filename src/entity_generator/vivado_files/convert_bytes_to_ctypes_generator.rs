use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct ConvertBytesToCTypes {}

impl ConvertBytesToCTypes {
    pub(crate) fn new() -> ConvertBytesToCTypes {
        ConvertBytesToCTypes {}
    }
}

impl GenerateVhdlCode for ConvertBytesToCTypes {
    fn template_name(&self) -> String {
        "convert_bytes_to_ctypes.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "convert_bytes_to_ctypes.h".to_string()
    }
}

#[cfg(test)]
mod convert_bytes_to_ctypes_file_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let cfile = ConvertBytesToCTypes::new();
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&cfile, &tera, &PathBuf::from("target/test_files"));
    }
}
