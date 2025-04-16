use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct FPGALowLevelCommunication {}

impl FPGALowLevelCommunication {
    pub(crate) fn new() -> FPGALowLevelCommunication {
        FPGALowLevelCommunication {}
    }
}

impl GenerateVhdlCode for FPGALowLevelCommunication {
    fn template_name(&self) -> String {
        "fpga_low_level_communication.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "fpga_low_level_communication.h".to_string()
    }
}

#[cfg(test)]
mod set_bits_in_fpga_file_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let cfile = FPGALowLevelCommunication::new();
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&cfile, &tera, &PathBuf::from("target/test_files"));
    }
}
