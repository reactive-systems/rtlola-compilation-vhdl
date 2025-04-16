#![deny(unsafe_code)]
#![warn(
    //missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use crate::entity_generator::high_level_controller::generate_timing_manager_entities;
use crate::entity_generator::low_level_controller::generate_evaluator;
use crate::entity_generator::queue::generate_vhdl_queue;
use crate::entity_generator::vivado_files::generate_vivado_files;
use crate::pre_procession::*;
use crate::static_constants::NUM_TEST_INPUTS;
use clap::{App, Arg, ArgGroup};
use entity_generator::*;
use rtlola_frontend::{ParserConfig, RtLolaMir};
use static_constants::*;
use std::path::PathBuf;
use tera::compile_templates;

pub(crate) mod entity_generator;
pub(crate) mod ir_extension;
pub(crate) mod static_constants;
pub(crate) mod vhdl_wrapper;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash, Copy)]
pub enum AdditionalFiles {
    Debug,
    Run,
    VivadoFiles,
}

#[derive(Debug, Clone)]
pub struct Config {
    ir: RtLolaMir,
    target: PathBuf,
    mode: bool,                                //true=online; false=offline
    additional_files: Option<AdditionalFiles>, //None=no additional files;
    templates: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let parse_matches = App::new("FPGA_StreamLAB")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("FPGA_StreamLAB is a tool to compile a Lola specification to VHDL code")
            .arg(Arg::with_name("SPEC").help("Sets the specification file to use").required(true).index(1))
            .arg(Arg::with_name("ONLINE").long("online").help("Use the system time for timestamps"))
            .arg(Arg::with_name("OFFLINE").long("offline").help("Use the timestamps from the input."))
            .group(ArgGroup::with_name("MODE").required(true).args(&["ONLINE", "OFFLINE"]))
            .arg(
                Arg::with_name("TARGET_DIR")
                    .help("Sets the directory, where to store the generated VHDL files")
                    .required(true)
                    .index(2),
            )
            .arg(
                Arg::with_name("TEMPLATES_DIR")
                    .help("Sets the directory, where to template files are stored")
                    .required(true)
                    .index(3),
            )
            .arg(
                Arg::with_name("VIVADO_FILES")
                    .short("v")
                    .long("vivado_files")
                    .help("Generates a folder with all files needed to recevive UDP-packages on the Evaluation Board"),
            )
            .arg(
                Arg::with_name("DEBUG")
                    .short("d")
                    .long("debug")
                    .help("Generates a build and a file to test the implementation with assertions."),
            )
            .arg(
                Arg::with_name("RUN")
                    .short("r")
                    .long("run")
                    .help("Generates a build and a file to run the implementation"),
            )
            .group(ArgGroup::with_name("ADDITIONAL_FILES").required(false).args(&["DEBUG", "RUN", "VIVADO_FILES"]))
            .get_matches_from(args);

        let filename = parse_matches.value_of("SPEC").map(|s| s.to_string()).unwrap();

        let ir = rtlola_frontend::parse(&ParserConfig::from_path(PathBuf::from(filename))?).unwrap();

        let mode = parse_matches.is_present("ONLINE");

        let target = parse_matches.value_of("TARGET_DIR").map(|s| s.to_string()).unwrap();

        let target = PathBuf::from(target);

        let templates = parse_matches.value_of("TEMPLATES_DIR").map(|s| s.to_string()).unwrap();

        let debug = parse_matches.is_present("DEBUG");
        let run = parse_matches.is_present("RUN");
        let vivado_files = parse_matches.is_present("VIVADO_FILES");

        let additional_files = if debug {
            Some(AdditionalFiles::Debug)
        } else if run {
            Some(AdditionalFiles::Run)
        } else if vivado_files {
            Some(AdditionalFiles::VivadoFiles)
        } else {
            None
        };

        Ok(Config { ir, target, mode, additional_files, templates })
    }

    pub fn generate_vhdl_files(self) {
        if !self.ir.triggers.is_empty() {
            unimplemented!("Triggers are currently not implemented. Please use output streams instead.")
        }
        let tera_files = self.templates.clone() + "/*";
        let tera = compile_templates!(&tera_files);

        // Check static constants:
        // Check if bit range is 16, 32, and 64 bit, starting with 0
        assert_eq!(
            FLOAT_16_HIGH - FLOAT_16_LOW,
            15,
            "Bit range of 16 bit fix-point representation is not 16 bit long!"
        );
        assert_eq!(
            FLOAT_32_HIGH - FLOAT_32_LOW,
            31,
            "Bit range of 32 bit fix-point representation is not 16 bit long!"
        );
        assert_eq!(
            FLOAT_64_HIGH - FLOAT_64_LOW,
            63,
            "Bit range of 64 bit fix-point representation is not 16 bit long!"
        );

        // Check if float values include each other
        assert!(
            FLOAT_64_LOW <= FLOAT_32_LOW && FLOAT_32_LOW <= FLOAT_16_LOW,
            "Fraction bits of small bit range must be contained in larger bit range"
        );
        assert!(
            FLOAT_64_HIGH >= FLOAT_32_HIGH && FLOAT_32_HIGH >= FLOAT_16_HIGH,
            "integer bits of small bit range must be contained in larger bit range"
        );

        generate_preprocessing(&self);
        VHDLGenerator::generate_and_create(&implementation::Implementation::new(&self.ir), &tera, &self.target);
        VHDLGenerator::generate_and_create(&monitor::Monitor::new(&self.ir), &tera, &self.target);
        VHDLGenerator::generate_and_create(
            &vhdl_array_package_declaration::ArrayPackageVHDL::new(),
            &tera,
            &self.target,
        );
        VHDLGenerator::generate_and_create(&vhdl_math_package_declaration::MathPackageVHDL::new(), &tera, &self.target);
        generate_evaluator(&self);
        generate_timing_manager_entities(&self);
        generate_vhdl_queue(&self);
        match self.additional_files {
            Some(AdditionalFiles::Debug) => {
                VHDLGenerator::generate_and_create(
                    &build_script::BuildScript::new(&self.ir, true, self.mode),
                    &tera,
                    &self.target,
                );
                VHDLGenerator::generate_and_create(
                    &run_test_generator::TestScriptVHDL::new(
                        &self.ir,
                        NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_OFFLINE,
                        NUM_TEST_INPUTS,
                    ),
                    &tera,
                    &self.target,
                )
            }
            Some(AdditionalFiles::Run) => {
                VHDLGenerator::generate_and_create(
                    &build_script::BuildScript::new(&self.ir, false, self.mode),
                    &tera,
                    &self.target,
                );
                VHDLGenerator::generate_and_create(
                    &run_impl_generator::RunScriptVHDL::new(
                        &self.ir,
                        NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_OFFLINE,
                        NUM_TEST_INPUTS,
                    ),
                    &tera,
                    &self.target,
                )
            }
            Some(AdditionalFiles::VivadoFiles) => {
                generate_vivado_files(&self);
            }
            None => {}
        }
    }
}
