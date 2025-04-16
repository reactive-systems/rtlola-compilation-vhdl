use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct TestScriptVHDL<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) num_cycles: u16,
    pub(crate) num_test_inputs: u32,
}

impl<'a> TestScriptVHDL<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, num_cycles: u16, num_test_inputs: u32) -> TestScriptVHDL<'a> {
        assert_eq!(num_cycles % 2, 0);
        TestScriptVHDL { ir, num_cycles, num_test_inputs }
    }
}

impl GenerateVhdlCode for TestScriptVHDL<'_> {
    fn template_name(&self) -> String {
        "run_test.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "run_test_wip.vhdl".to_string()
    }
}

impl Serialize for TestScriptVHDL<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_test_script_setup();
        let mut s = serializer.serialize_struct("Testscript", 8)?;
        s.serialize_field("input", &setup.input.concat())?;
        s.serialize_field("output", &setup.output.concat())?;
        s.serialize_field("signals", &setup.signals.concat())?;
        s.serialize_field("monitor_instantiation_input", &setup.monitor_instantiation_input.concat())?;
        s.serialize_field("monitor_instantiation_output", &setup.monitor_instantiation_output.concat())?;
        s.serialize_field("set_inputs", &setup.set_inputs.concat())?;
        s.serialize_field("num_input_cycle", &((self.num_cycles + 1) * 2))?;
        s.serialize_field("num_clock_cycles", &((self.num_cycles + 1) * 2))?;
        s.serialize_field("num_test_inputs", &(self.num_test_inputs))?;
        s.serialize_field("input_definition", &setup.input_definition.concat())?;
        s.serialize_field("expected_output", &setup.expected_output.concat())?;
        s.serialize_field("disable_inputs", &setup.disable_inputs.concat())?;
        s.serialize_field("asserts", &setup.asserts.concat())?;
        s.end()
    }
}

pub(crate) struct TestScriptSetup {
    pub(crate) input: Vec<String>,
    pub(crate) output: Vec<String>,
    pub(crate) signals: Vec<String>,
    pub(crate) monitor_instantiation_input: Vec<String>,
    pub(crate) monitor_instantiation_output: Vec<String>,
    pub(crate) set_inputs: Vec<String>,
    pub(crate) input_definition: Vec<String>,
    pub(crate) disable_inputs: Vec<String>,
    pub(crate) expected_output: Vec<String>,
    pub(crate) asserts: Vec<String>,
}

impl TestScriptSetup {
    pub(crate) fn new() -> TestScriptSetup {
        TestScriptSetup {
            input: Vec::new(),
            output: Vec::new(),
            signals: Vec::new(),
            monitor_instantiation_input: Vec::new(),
            monitor_instantiation_output: Vec::new(),
            set_inputs: Vec::new(),
            input_definition: Vec::new(),
            expected_output: Vec::new(),
            disable_inputs: Vec::new(),
            asserts: Vec::new(),
        }
    }
}

impl TestScriptVHDL<'_> {
    fn generate_test_script_setup(&self) -> TestScriptSetup {
        let range_inputs: Vec<u32> = (0..self.num_test_inputs).collect();
        let mut setup = TestScriptSetup::new();
        setup
            .set_inputs
            .push("\n\t\t\t\ttime_data <= std_logic_vector(time_test_data(I) * time_unit_multiplication);".to_string());
        setup.set_inputs.push("\n\t\t\t\tnew_input_in <= new_input_test_data(I);".to_string());
        let time_inputs: Vec<String> =
            range_inputs.iter().map(|_| ",\n\t\tto_unsigned(0000, 32)".to_string()).collect();
        setup.input_definition.push(format!(
            "\n\tconstant time_test_data : unsigned32_array(0 to {}) := (\n\t\tto_unsigned(0000,32){}\n\t);",
            self.num_test_inputs,
            time_inputs.concat()
        ));
        let num_input_inputs: Vec<String> = range_inputs.iter().map(|_| ",'0'".to_string()).collect();
        setup.input_definition.push(format!(
            "\n\tconstant new_input_test_data : bit_array(0 to {}) := ('0'{});",
            self.num_test_inputs,
            num_input_inputs.concat()
        ));
        setup.disable_inputs.push("new_input_in <= '0';".to_string());
        self.ir.inputs.iter().for_each(|cur| {
            let vhdl_type = get_vhdl_initial_type(&cur.ty);
            let vhdl_array_type = generate_vhdl_array_type(&cur.ty, self.num_test_inputs, false);
            let default_init_value = generate_vhdl_type_default_initialisation(&cur.ty);
            let default_value = generate_vhdl_type_default(&cur.ty, format!("{}_data", cur.name));
            setup.input.push(format!("\n\t\t\t{}_data_in : in {};\n\t\t\t{}_data_in_new_input : in std_logic;", cur.name, vhdl_type, cur.name));
            setup.output.push(format!("\n\t\t\t{}_stream : out {};", cur.name, vhdl_type));
            setup.signals.push(format!("\n\tsignal {}_data : {} := {};\n\tsignal {}_data_new_input : std_logic := '0';\n\tsignal {}_data_out : {} := {};", cur.name, vhdl_type, default_init_value, cur.name, cur.name, vhdl_type, default_init_value));
            setup.monitor_instantiation_input.push(format!("\n\t\t\t\t\t{}_data_in => {}_data,\n\t\t\t\t\t{}_data_in_new_input => {}_data_new_input,", cur.name, cur.name, cur.name, cur.name));
            setup.monitor_instantiation_output.push(format!("\n\t\t\t\t\t{}_stream => {}_data_out,", cur.name, cur.name));
            let convert_pre = match cur.ty {
                Type::Bool => "",
                Type::Float(_) => "to_slv",
                _ => "std_logic_vector"
            };
            setup.set_inputs.push(format!("\n\t\t\t\t{}_data <= {}({}_test_data(I));\n\t\t\t\t{}_data_new_input <= {}_new_input_test_data(I);", cur.name, convert_pre, cur.name, cur.name, cur.name));
            let test_data: Vec<String> = range_inputs.iter().map(|_| format!(",\n\t\t{}", default_value)).collect();
            setup.input_definition.push(format!(
                "\n\tconstant {}_test_data : {} := (\n\t\t{}{}\n\t);", cur.name, vhdl_array_type, default_value,test_data.concat()
            ));
            let new_input_data: Vec<String> = range_inputs.iter().map(|_| ",'0'".to_string()).collect();
            setup.input_definition.push(format!(
                "\n\tconstant {}_new_input_test_data : bit_array(0 to {}) := ('0'{});", cur.name, self.num_test_inputs,new_input_data.concat()
            ));
            setup.disable_inputs.push(format!("\n\t\t\t{}_data_new_input <= '0';", cur.name));
            setup.asserts.push(format!("\n\t\t\t\tassert {}_data_out = {}_data\n\t\t\t\t\treport \"{}_data differ\"\n\t\t\t\t\tseverity Error;", cur.name, cur.name, cur.name));
        });
        self.ir.outputs.iter().for_each(|cur| {
            let vhdl_type = get_vhdl_initial_type(&cur.ty);
            let vhdl_array_type = generate_vhdl_array_type(&cur.ty, self.num_test_inputs, false);
            let default_value = generate_vhdl_type_default(&cur.ty, format!("{}_data_out", cur.name));
            let default_init_value = generate_vhdl_type_default_initialisation(&cur.ty);
            setup.output.push(format!("\n\t\t\t{}_stream : out {};", cur.name, vhdl_type));
            setup.signals.push(format!("\n\tsignal {}_data_out : {} := {};", cur.name, vhdl_type, default_init_value));
            setup.monitor_instantiation_output.push(format!("\n\t\t\t\t{}_stream => {}_data_out,", cur.name, cur.name));
            let test_data: Vec<String> = range_inputs.iter().map(|_| format!(",\n\t\t{}", default_value)).collect();
            setup.expected_output.push(format!(
                "\n\tconstant exp_{}_data : {} := (\n\t\t{}{}\n\t);", cur.name, vhdl_array_type, default_value,test_data.concat()
            ));
            let convert_pre = match cur.ty {
                Type::Bool => "",
                Type::Float(_) => "to_slv",
                _ => "std_logic_vector"
            };
            setup.asserts.push(format!("\n\t\t\t\tassert {}_data_out = {}(exp_{}_data(I))\n\t\t\t\t\treport \"{}_data differ at iteration I = \" & integer'image(I)\n\t\t\t\t\tseverity Error;", cur.name, convert_pre, cur.name, cur.name));
        });
        setup
    }
}

#[cfg(test)]
mod run_test_generator_test {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use crate::static_constants::NUM_TEST_INPUTS;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_run_test_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let test_script = TestScriptVHDL::new(&lola_instance, 16, NUM_TEST_INPUTS);
        let tera: Tera = compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&test_script, &tera, &PathBuf::from("target/test_files"));
    }
}
