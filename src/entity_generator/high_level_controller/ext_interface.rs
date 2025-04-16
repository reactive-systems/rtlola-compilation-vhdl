use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::{InputStream, Type};
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct ExtInterface<'a> {
    pub(crate) inputs: &'a Vec<InputStream>,
}

impl<'a> ExtInterface<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> ExtInterface<'a> {
        ExtInterface { inputs: &ir.inputs }
    }
}

impl GenerateVhdlCode for ExtInterface<'_> {
    fn template_name(&self) -> String {
        "extInterface.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "extInterface.vhdl".to_string()
    }
}

impl Serialize for ExtInterface<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_input_manager_setup();
        let mut s = serializer.serialize_struct("InputManager", 7)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("converted_signals", &setup.converted_signals.concat())?;
        s.serialize_field("signal_default_assignment", &setup.signal_default_assignment.concat())?;
        s.serialize_field("converts", &setup.converts.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;
        s.serialize_field("print_input_streams", &setup.print_input_streams.concat())?;
        s.end()
    }
}

pub(crate) struct ExtInterfaceSetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) converted_signals: Vec<String>,
    pub(crate) signal_default_assignment: Vec<String>,
    pub(crate) converts: Vec<String>,
    pub(crate) final_mapping: Vec<String>,
    pub(crate) print_input_streams: Vec<String>,
}

impl ExtInterfaceSetup {
    fn new() -> ExtInterfaceSetup {
        ExtInterfaceSetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            converted_signals: Vec::new(),
            signal_default_assignment: Vec::new(),
            converts: Vec::new(),
            final_mapping: Vec::new(),
            print_input_streams: Vec::new(),
        }
    }
}

impl ExtInterface<'_> {
    fn generate_input_manager_setup(&self) -> ExtInterfaceSetup {
        let mut setup = ExtInterfaceSetup::new();
        self.inputs.iter().for_each(|cur| {
            let annotation = format!("input {} : {}", cur.name, cur.ty);
            if cur.name.as_str() != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_push_in : in std_logic;",
                    cur.name,
                    get_vhdl_initial_type(&cur.ty),
                    cur.name
                ));
                setup.outputs.push(format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_push_out : out std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.converted_signals.push(format!(
                    "\n\tsignal {}_parsed : {};\n\tsignal {}_push_delayed : std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.signal_default_assignment.push(format!(
                    "\n\t\t\t{}_parsed <= {};\n\t\t\t{}_push_delayed <= '0';",
                    cur.name,
                    generate_vhdl_type_default_initialisation(&cur.ty),
                    cur.name
                ));
                setup.converts.push(format!(
                    "\n\t\t\t--* {}\n\t\t\t{};\n\t\t\t{}_push_delayed <= {}_push_in;",
                    annotation,
                    ExtInterface::convert_single_input(cur),
                    cur.name,
                    cur.name
                ));
                setup.final_mapping.push(format!(
                    "\n\t{}_data_out <= {}_parsed;\n\t{}_push_out <= {}_push_delayed;",
                    cur.name, cur.name, cur.name, cur.name
                ));
            }
            setup.print_input_streams.push(format!("\n--* - {}", annotation));
        });
        setup
    }

    fn convert_single_input(input: &InputStream) -> String {
        match input.ty {
            Type::Int(_) => format!("{}_parsed <= signed({}_data_in)", input.name, input.name),
            Type::UInt(_) => format!("{}_parsed <= unsigned({}_data_in)", input.name, input.name),
            Type::Float(fl_ty) => {
                let (high, low) = get_float_range(fl_ty);
                format!("{}_parsed <= to_sfixed({}_data_in, {}, {})", input.name, input.name, high, low)
            }
            Type::Bool => format!("{}_parsed <= {}_data_in", input.name, input.name),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod input_manager_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_input_manager_file() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = ExtInterface::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&input_manager, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn test_input_manager() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = ExtInterface::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&input_manager, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result_entity = &result[first_pos..last_pos];
        let result_entity: Vec<&str> = result_entity.split("\n").collect();
        //check lines
        assert_eq!(result_entity[4].trim(), "a_data_in : in std_logic;");
        assert_eq!(result_entity[6].trim(), "b_data_in : in std_logic_vector(7 downto 0);");
        assert_eq!(result_entity[8].trim(), "a_data_out : out std_logic;");
        assert_eq!(result_entity[10].trim(), "b_data_out : out signed(7 downto 0);");
        //take signal
        let first_pos = result.find("architecture").expect("expected entity declaration");
        let last_pos = result.find("begin").expect("expected begin process");
        let result_signal = &result[first_pos..last_pos];
        let result_signal: Vec<&str> = result_signal.split("\n").collect();
        //check lines
        assert_eq!(result_signal[4].trim(), "signal a_parsed : std_logic;");
        assert_eq!(result_signal[6].trim(), "signal b_parsed : signed(7 downto 0);");
    }
}
