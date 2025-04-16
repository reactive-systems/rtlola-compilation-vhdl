use crate::entity_generator::*;
use crate::vhdl_wrapper::type_serialize::*;
use crate::Config;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) mod evaluator_entity;
pub(crate) mod input_entity;
pub(crate) mod output_entity;
pub(crate) mod sliding_window;

pub(crate) fn generate_evaluator(config: &Config) {
    let mut target = config.target.clone();
    target.push("llc/");
    let tera_files = config.templates.clone() + "/low_level_controller/*";
    let tera = tera::compile_templates!(&tera_files);
    for input in &config.ir.inputs {
        VHDLGenerator::generate_and_create(&input_entity::InputStreamVHDL::new(input, &config.ir), &tera, &target);
    }
    for output in &config.ir.outputs {
        VHDLGenerator::generate_and_create(&output_entity::OutputStreamVHDL::new(output, &config.ir), &tera, &target);
    }
    for sw in &config.ir.sliding_windows {
        sliding_window::generate_sliding_window(sw, config);
    }
    VHDLGenerator::generate_and_create(
        &evaluator_entity::Evaluator::new(&config.ir, config.templates.clone()),
        &tera,
        &target,
    );
    VHDLGenerator::generate_and_create(&LowLevelController::new(&config.ir), &tera, &target);
}

pub(crate) struct LowLevelController<'a> {
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> LowLevelController<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> LowLevelController<'a> {
        LowLevelController { ir }
    }
}

impl GenerateVhdlCode for LowLevelController<'_> {
    fn template_name(&self) -> String {
        "low_level_controller.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "low_level_controller.vhdl".to_string()
    }
}

impl Serialize for LowLevelController<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_evaluator_setup();
        let mut s = serializer.serialize_struct("Monitor", 6)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("inputs_evaluator", &setup.inputs_evaluator.concat())?;
        s.serialize_field("outputs_evaluator", &setup.outputs_evaluator.concat())?;
        s.serialize_field("input_evaluator_instance", &setup.input_evaluator_instance.concat())?;
        s.serialize_field("output_evaluator_instance", &setup.output_evaluator_instance.concat())?;
        s.end()
    }
}

pub(crate) struct LowLevelControllerSetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) inputs_evaluator: Vec<String>,
    pub(crate) outputs_evaluator: Vec<String>,
    pub(crate) input_evaluator_instance: Vec<String>,
    pub(crate) output_evaluator_instance: Vec<String>,
}

impl LowLevelControllerSetup {
    fn new() -> LowLevelControllerSetup {
        LowLevelControllerSetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            inputs_evaluator: Vec::new(),
            outputs_evaluator: Vec::new(),
            input_evaluator_instance: Vec::new(),
            output_evaluator_instance: Vec::new(),
        }
    }
}

impl LowLevelController<'_> {
    fn generate_evaluator_setup(&self) -> LowLevelControllerSetup {
        let mut setup = LowLevelControllerSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{} : in {};\n\t\t{}_en : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.inputs_evaluator.push(format!(
                    "\n\t\t\t{} : in {};\n\t\t\t{}_en : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup
                    .input_evaluator_instance
                    .push(format!("\n\t\t\t{} => {},\n\t\t\t{}_en => {}_en,", cur.name, cur.name, cur.name, cur.name));
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            setup.inputs.push(format!("\n\t\t{}_en : in std_logic;", cur.name));
            setup.outputs.push(format!("\n\t\t{} : out {};", cur.name, get_vhdl_type(&cur.ty)));
            setup.inputs_evaluator.push(format!("\n\t\t\t{}_en : in std_logic;", cur.name));
            setup.outputs_evaluator.push(format!("\n\t\t\t{} : out {};", cur.name, get_vhdl_type(&cur.ty)));
            setup.input_evaluator_instance.push(format!("\n\t\t\t{}_en => {}_en,", cur.name, cur.name));
            setup.output_evaluator_instance.push(format!("\n\t\t\t{} => {},", cur.name, cur.name));
        });
        setup
    }
}

#[cfg(test)]
mod monitor_test {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_evaluator_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let evaluator = LowLevelController::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        VHDLGenerator::generate_and_create(&evaluator, &tera, &PathBuf::from("target/test_files"));
    }
}
