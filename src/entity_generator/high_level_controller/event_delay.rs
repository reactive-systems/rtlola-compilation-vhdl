use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::{mir::InputStream, RtLolaMir};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct EventDelay<'a> {
    pub(crate) inputs: &'a Vec<InputStream>,
}

impl<'a> EventDelay<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> EventDelay<'a> {
        EventDelay { inputs: &ir.inputs }
    }
}

impl GenerateVhdlCode for EventDelay<'_> {
    fn template_name(&self) -> String {
        "event_delay.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "event_delay.vhdl".to_string()
    }
}

impl Serialize for EventDelay<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = &self.generate_event_manager_setup();
        let mut s = serializer.serialize_struct("EventManager", 7)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("delay_registers", &setup.delay_registers.concat())?;
        s.serialize_field("defaults", &setup.defaults.concat())?;
        s.serialize_field("delay_mapping", &setup.delay_mapping.concat())?;
        s.serialize_field("zero_mapping", &setup.zero_mapping.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;
        s.serialize_field("print_input_streams", &setup.print_input_streams.concat())?;
        s.end()
    }
}

pub(crate) struct EventDelaySetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) delay_registers: Vec<String>,
    pub(crate) defaults: Vec<String>,
    pub(crate) delay_mapping: Vec<String>,
    pub(crate) zero_mapping: Vec<String>,
    pub(crate) final_mapping: Vec<String>,
    pub(crate) print_input_streams: Vec<String>,
}

impl EventDelaySetup {
    fn new() -> EventDelaySetup {
        EventDelaySetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            delay_registers: Vec::new(),
            defaults: Vec::new(),
            delay_mapping: Vec::new(),
            zero_mapping: Vec::new(),
            final_mapping: Vec::new(),
            print_input_streams: Vec::new(),
        }
    }
}

impl EventDelay<'_> {
    fn generate_event_manager_setup(&self) -> EventDelaySetup {
        let mut setup = EventDelaySetup::new();
        self.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_push_in : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.outputs.push(format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_push_out : out std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.delay_registers.push(format!(
                    "\n\tsignal {}_data_delayed : {};\n\tsignal {}_push_delayed : std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.defaults.push(format!(
                    "\n\t\t\t{}_data_delayed <= {};\n\t\t\t{}_push_delayed <= '0';",
                    cur.name,
                    generate_vhdl_type_default(&cur.ty, format!("{}_data_delayed", cur.name)),
                    cur.name
                ));
                setup.zero_mapping.push(format!(
                    "\n\t\t\t\t{}_data_delayed <= {};\n\t\t\t\t{}_push_delayed <= '0';",
                    cur.name,
                    generate_vhdl_type_default(&cur.ty, format!("{}_data_delayed", cur.name)),
                    cur.name
                ));
                setup.delay_mapping.push(format!(
                    "\n\t\t\t\t{}_data_delayed <= {}_data_in;\n\t\t\t\t{}_push_delayed <= {}_push_in;",
                    cur.name, cur.name, cur.name, cur.name,
                ));
                setup.final_mapping.push(format!(
                    "\n\t{}_data_out <= {}_data_delayed;\n\t{}_push_out <= {}_push_delayed;",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.print_input_streams.push(format!("\n--* - {} : {} *--", cur.name, cur.ty));
            }
        });
        setup
    }
}

#[cfg(test)]
mod event_manager_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_event_manager_file() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = EventDelay::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&input_manager, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn test_event_manager() {
        let example_file_content = "input a : Bool\ninput b : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = EventDelay::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&input_manager, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result_entity = &result[first_pos..last_pos];
        let result_entity: Vec<&str> = result_entity.split("\n").collect();
        //check lines
        assert_eq!(result_entity[4].trim(), "a_data_in : in std_logic;");
        assert_eq!(result_entity[6].trim(), "b_data_in : in signed(7 downto 0);");
        assert_eq!(result_entity[10].trim(), "a_data_out : out std_logic;");
        assert_eq!(result_entity[12].trim(), "b_data_out : out signed(7 downto 0);");
        //take signal
        let first_pos = result.find("architecture").expect("expected entity declaration");
        let last_pos = result.find("begin").expect("expected begin process");
        let result_signal = &result[first_pos..last_pos];
        let result_signal: Vec<&str> = result_signal.split("\n").collect();
        //check lines
        assert_eq!(result_signal[5].trim(), "signal a_data_delayed : std_logic;");
        assert_eq!(result_signal[6].trim(), "signal a_push_delayed : std_logic;");
        assert_eq!(result_signal[7].trim(), "signal b_data_delayed : signed(7 downto 0);");
        assert_eq!(result_signal[8].trim(), "signal b_push_delayed : std_logic;");
        //take finial mapping
        let first_pos = result.find("end process;").expect("expected end process");
        let last_pos = result.find("end behavioral;").expect("expected end behavioral");
        let result_final_mapping = &result[first_pos..last_pos];
        let result_final_mapping: Vec<&str> = result_final_mapping.split("\n").collect();
        //check lines
        assert_eq!(result_final_mapping[4].trim(), "a_data_out <= a_data_delayed;");
        assert_eq!(result_final_mapping[5].trim(), "a_push_out <= a_push_delayed;");
        assert_eq!(result_final_mapping[6].trim(), "b_data_out <= b_data_delayed;");
        assert_eq!(result_final_mapping[7].trim(), "b_push_out <= b_push_delayed;");
    }
}
