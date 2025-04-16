use crate::entity_generator::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct Implementation<'a> {
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> Implementation<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> Implementation<'a> {
        Implementation { ir }
    }
}

impl GenerateVhdlCode for Implementation<'_> {
    fn template_name(&self) -> String {
        "implementation.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "implementation.vhdl".to_string()
    }
}

impl Serialize for Implementation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_implementation_setup();
        let mut s = serializer.serialize_struct("Monitor", 11)?;
        s.serialize_field("input", &setup.input.concat())?;
        s.serialize_field("output", &setup.output.concat())?;
        s.serialize_field(
            "input_pre_processing_component_input",
            &setup.input_pre_processing_component_input.concat(),
        )?;
        s.serialize_field(
            "input_pre_processing_component_output",
            &setup.input_pre_processing_component_output.concat(),
        )?;
        s.serialize_field("monitor_component_input", &setup.monitor_component_input.concat())?;
        s.serialize_field("monitor_component_output", &setup.monitor_component_output.concat())?;
        s.serialize_field("registers", &setup.registers.concat())?;
        s.serialize_field(
            "input_pre_processing_instantiation_input",
            &setup.input_pre_processing_instantiation_input.concat(),
        )?;
        s.serialize_field(
            "input_pre_processing_instantiation_output",
            &setup.input_pre_processing_instantiation_output.concat(),
        )?;
        s.serialize_field("monitor_instantiation_input", &setup.monitor_instantiation_input.concat())?;
        s.serialize_field("monitor_instantiation_output", &setup.monitor_instantiation_output.concat())?;
        s.end()
    }
}

struct ImplementationSetup {
    input: Vec<String>,
    output: Vec<String>,
    input_pre_processing_component_input: Vec<String>,
    input_pre_processing_component_output: Vec<String>,
    monitor_component_input: Vec<String>,
    monitor_component_output: Vec<String>,
    registers: Vec<String>,
    input_pre_processing_instantiation_input: Vec<String>,
    input_pre_processing_instantiation_output: Vec<String>,
    monitor_instantiation_input: Vec<String>,
    monitor_instantiation_output: Vec<String>,
}

impl ImplementationSetup {
    fn new() -> ImplementationSetup {
        ImplementationSetup {
            input: Vec::new(),
            output: Vec::new(),
            input_pre_processing_component_input: Vec::new(),
            input_pre_processing_component_output: Vec::new(),
            monitor_component_input: Vec::new(),
            monitor_component_output: Vec::new(),
            registers: Vec::new(),
            input_pre_processing_instantiation_input: Vec::new(),
            input_pre_processing_instantiation_output: Vec::new(),
            monitor_instantiation_input: Vec::new(),
            monitor_instantiation_output: Vec::new(),
        }
    }
}

impl Implementation<'_> {
    fn generate_implementation_setup(&self) -> ImplementationSetup {
        let mut setup = ImplementationSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
                setup.input.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_data_in_new_input : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.output.push(format!("\n\t\t{}_stream: out {};", cur.name, vhdl_init_type));
                setup.input_pre_processing_component_input.push(format!(
                    "\n\t\t\t{}_in : in {};\n\t\t\t{}_in_new_input : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.input_pre_processing_component_output.push(format!(
                    "\n\t\t\t{}_out : out {};\n\t\t\t{}_out_new_input : out std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.monitor_component_input.push(format!(
                    "\n\t\t\t{}_data_in : in {};\n\t\t\t{}_data_in_new_input : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.monitor_component_output.push(format!("\n\t\t\t{}_stream: out {};", cur.name, vhdl_init_type));
                setup.registers.push(format!(
                    "\n\tsignal {}_data_reg : {};\n\tsignal {}_data_reg_new_input : std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.input_pre_processing_instantiation_input.push(format!(
                    "\n\t\t\t{}_in => {}_data_in,\n\t\t\t{}_in_new_input => {}_data_in_new_input,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.input_pre_processing_instantiation_output.push(format!(
                    "\n\t\t\t{}_out => {}_data_reg,\n\t\t\t{}_out_new_input => {}_data_reg_new_input,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.monitor_instantiation_input.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_reg,\n\t\t\t{}_data_in_new_input => {}_data_reg_new_input,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.monitor_instantiation_output.push(format!("\n\t\t\t{}_stream => {}_stream,", cur.name, cur.name));
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
            setup.output.push(format!("\n\t\t{}_stream: out {};", cur.name, vhdl_init_type));
            setup.monitor_component_output.push(format!("\n\t\t\t{}_stream: out {};", cur.name, vhdl_init_type));
            setup.monitor_instantiation_output.push(format!("\n\t\t\t{}_stream => {}_stream,", cur.name, cur.name));
        });
        setup
    }
}

#[cfg(test)]
mod implementation_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_implementation_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let implementation = Implementation::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&implementation, &tera, &PathBuf::from("target/test_files"))
    }
}
