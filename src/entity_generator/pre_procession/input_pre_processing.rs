use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::{mir::InputStream, RtLolaMir};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct InputPreProcessing<'a> {
    pub(crate) inputs: &'a Vec<InputStream>,
}

impl<'a> InputPreProcessing<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> InputPreProcessing<'a> {
        InputPreProcessing { inputs: &ir.inputs }
    }
}

impl GenerateVhdlCode for InputPreProcessing<'_> {
    fn template_name(&self) -> String {
        "input_pre_processing.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "input_pre_processing.vhdl".to_string()
    }
}

impl Serialize for InputPreProcessing<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_delay_register_setup();
        let mut s = serializer.serialize_struct("InputManager", 6)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("registers", &setup.registers.concat())?;
        s.serialize_field("defaults", &setup.defaults.concat())?;
        s.serialize_field("delay", &setup.delay.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;
        s.end()
    }
}

pub(crate) struct InputPreProcessingSetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) registers: Vec<String>,
    pub(crate) defaults: Vec<String>,
    pub(crate) delay: Vec<String>,
    pub(crate) final_mapping: Vec<String>,
}

impl InputPreProcessingSetup {
    fn new() -> InputPreProcessingSetup {
        InputPreProcessingSetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            registers: Vec::new(),
            defaults: Vec::new(),
            delay: Vec::new(),
            final_mapping: Vec::new(),
        }
    }
}

impl InputPreProcessing<'_> {
    fn generate_delay_register_setup(&self) -> InputPreProcessingSetup {
        let mut setup = InputPreProcessingSetup::new();
        self.inputs.iter().for_each(|cur| {
            if cur.name != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{}_in : in {};\n\t\t{}_in_new_input : in std_logic;",
                    cur.name,
                    get_vhdl_initial_type(&cur.ty),
                    cur.name
                ));
                setup.outputs.push(format!(
                    "\n\t\t{}_out : out {};\n\t\t{}_out_new_input : out std_logic;",
                    cur.name,
                    get_vhdl_initial_type(&cur.ty),
                    cur.name
                ));
                setup.registers.push(format!(
                    "\n\tsignal {}_reg : {};\n\tsignal {}_reg_new_input : std_logic;",
                    cur.name,
                    get_vhdl_initial_type(&cur.ty),
                    cur.name
                ));
                setup.defaults.push(format!(
                    "\n\t\t\t{}_reg <= {};\n\t\t\t{}_reg_new_input <= '0';",
                    cur.name,
                    generate_vhdl_type_default_initialisation(&cur.ty),
                    cur.name
                ));
                setup.delay.push(format!(
                    "\n\t\t\t\t{}_reg <= {}_in;\n\t\t\t\t{}_reg_new_input <= {}_in_new_input;",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.final_mapping.push(format!(
                    "\n\t{}_out <= {}_reg;\n\t{}_out_new_input <= {}_reg_new_input;",
                    cur.name, cur.name, cur.name, cur.name
                ));
            }
        });
        setup
    }
}

#[cfg(test)]
mod input_pre_processing_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_input_pre_processing_file() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_pre_processing = InputPreProcessing::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/pre_processing/*");
        VHDLGenerator::generate_and_create(&input_pre_processing, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn test_input_pre_processing() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_pre_processing = InputPreProcessing::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/pre_processing/*");
        let result = VHDLGenerator::generate(&input_pre_processing, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result_entity = &result[first_pos..last_pos];
        let result_entity: Vec<&str> = result_entity.split("\n").collect();
        //check lines
        assert_eq!(result_entity[4].trim(), "a_in : in std_logic;");
        assert_eq!(result_entity[6].trim(), "b_in : in std_logic_vector(7 downto 0);");
        assert_eq!(result_entity[10].trim(), "a_out : out std_logic;");
        assert_eq!(result_entity[12].trim(), "b_out : out std_logic_vector(7 downto 0);");
        //take signal
        let first_pos = result.find("architecture").expect("expected entity declaration");
        let last_pos = result.find("begin").expect("expected begin process");
        let result_signal = &result[first_pos..last_pos];
        let result_signal: Vec<&str> = result_signal.split("\n").collect();
        //check lines
        assert_eq!(result_signal[4].trim(), "signal a_reg : std_logic;");
        assert_eq!(result_signal[6].trim(), "signal b_reg : std_logic_vector(7 downto 0);");
    }
}
