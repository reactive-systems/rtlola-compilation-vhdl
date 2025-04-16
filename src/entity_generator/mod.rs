pub(crate) mod build_script;
pub(crate) mod high_level_controller;
pub(crate) mod implementation;
pub(crate) mod low_level_controller;
pub(crate) mod monitor;
pub(crate) mod pre_procession;
pub(crate) mod queue;
pub(crate) mod run_impl_generator;
pub(crate) mod run_test_generator;
pub(crate) mod vhdl_array_package_declaration;
pub(crate) mod vhdl_math_package_declaration;
pub(crate) mod vivado_files;

use rtlola_frontend::RtLolaMir;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tera::Tera;

pub(crate) trait GenerateVhdlCode: Serialize {
    fn template_name(&self) -> String;
    fn file_name(&self) -> String;
}

pub(crate) struct VHDLGenerator {}

impl VHDLGenerator {
    pub(crate) fn generate<T: GenerateVhdlCode>(entity: &T, tera: &Tera) -> String {
        let template_name = entity.template_name();
        tera.render(template_name.as_str(), entity).expect("Bug in code: Cannot render object.")
    }

    pub(crate) fn generate_and_create<T: GenerateVhdlCode>(entity: &T, tera: &Tera, dir_path: &PathBuf) {
        let mut path = dir_path.clone();
        let file_name = entity.file_name();
        let file_content = VHDLGenerator::generate(entity, tera);
        fs::create_dir_all(dir_path).expect("Cannot create output directory.");
        path.push(file_name);
        let mut file = File::create(path).expect("Cannot create output file.");
        file.write_all(&file_content.into_bytes()).expect("Cannot write to output file.");
    }
}
