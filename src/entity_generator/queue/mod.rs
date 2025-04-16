use crate::entity_generator::{GenerateVhdlCode, VHDLGenerator};
use crate::vhdl_wrapper::type_serialize::*;
use crate::Config;
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct VHDLQueue<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) queue_length: u32,
}

pub(crate) fn generate_vhdl_queue(config: &Config) {
    let mut target = config.target.clone();
    target.push("queue/");
    let tera_files = config.templates.clone() + "/queue/*";
    let tera = tera::compile_templates!(&tera_files);
    VHDLGenerator::generate_and_create(&VHDLQueue::new(&config.ir), &tera, &target);
}

impl<'a> VHDLQueue<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> VHDLQueue<'a> {
        VHDLQueue { ir, queue_length: 1 }
    }
}

impl GenerateVhdlCode for VHDLQueue<'_> {
    fn template_name(&self) -> String {
        "queue.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "queue.vhdl".to_string()
    }
}

impl Serialize for VHDLQueue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_queue_setup();
        let mut s = serializer.serialize_struct("Queue", 9)?;
        s.serialize_field("inputs", &setup.input.concat())?;
        s.serialize_field("outputs", &setup.output.concat())?;
        s.serialize_field("registers", &setup.registers.concat())?;
        s.serialize_field("defaults", &setup.defaults.concat())?;
        s.serialize_field("pop", &setup.pop.concat())?;
        s.serialize_field("push", &setup.push.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;
        s.serialize_field("array_size", &self.queue_length)?;
        s.serialize_field("length_queue", &(self.queue_length + 1))?;
        s.end()
    }
}

struct VHDLQueueSetup {
    input: Vec<String>,
    output: Vec<String>,
    registers: Vec<String>,
    defaults: Vec<String>,
    pop: Vec<String>,
    push: Vec<String>,
    final_mapping: Vec<String>,
}

impl VHDLQueueSetup {
    fn new() -> VHDLQueueSetup {
        VHDLQueueSetup {
            input: Vec::new(),
            output: Vec::new(),
            registers: Vec::new(),
            defaults: Vec::new(),
            pop: Vec::new(),
            push: Vec::new(),
            final_mapping: Vec::new(),
        }
    }
}

impl VHDLQueue<'_> {
    fn generate_queue_setup(&self) -> VHDLQueueSetup {
        let mut setup = VHDLQueueSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
            setup.input.push(format!(
                "\n\t\t{}_data_in : in {};\n\t\t{}_en_in : in std_logic;",
                cur.name,
                get_vhdl_type(&cur.ty),
                cur.name
            ));
            setup.output.push(format!(
                "\n\t\t{}_data_out : out {};\n\t\t{}_en_out : out std_logic;",
                cur.name,
                get_vhdl_type(&cur.ty),
                cur.name
            ));
            setup.registers.push(format!("\n\tsignal {}_data_reg : {};\n\tsignal {}_en_reg : bit_array({} downto 0);\n\tsignal {}_data : {};\n\tsignal {}_en: std_logic;", cur.name, generate_vhdl_array_type_downwards(&cur.ty, self.queue_length), cur.name, self.queue_length, cur.name, get_vhdl_type(&cur.ty), cur.name));
            setup.defaults.push(format!("\n\t\t\t{}_data_reg({}_data_reg'high downto 0) <= {};\n\t\t\t{}_en_reg({}_en_reg'high downto 0) <= (others => '0');\n\t\t\t{}_data <= {};\n\t\t\t{}_en <= '0';", cur.name, cur.name, generate_vhdl_array_default_initialisation(&cur.ty), cur.name, cur.name, cur.name, generate_vhdl_type_default_initialisation(&cur.ty), cur.name));
            setup.pop.push(format!(
                "\n\t\t\t\t\t{}_data <= {}_data_reg(size-1);\n\t\t\t\t\t{}_en <= {}_en_reg(size-1);",
                cur.name, cur.name, cur.name, cur.name
            ));
            setup.push.push(format!("\n\t\t\t\t\t{}_data_reg <= {}_data_reg({}_data_reg'high - 1 downto 0) & {}_data_in;\n\t\t\t\t\t{}_en_reg <= {}_en_reg({}_en_reg'high - 1 downto 0) & {}_en_in;", cur.name, cur.name, cur.name, cur.name, cur.name, cur.name, cur.name, cur.name));
            setup.final_mapping.push(format!("\n\t{}_data_out <= {}_data;\n\t{}_en_out <= {}_en;", cur.name, cur.name, cur.name, cur.name));}
        });
        self.ir.outputs.iter().for_each(|cur| {
            setup.input.push(format!("\n\t\t{}_en_in : in std_logic;", cur.name));
            setup.output.push(format!("\n\t\t{}_en_out : out std_logic;", cur.name));
            setup.registers.push(format!(
                "\n\tsignal {}_en_reg : bit_array({} downto 0);\n\tsignal {}_en : std_logic;",
                cur.name, self.queue_length, cur.name
            ));
            setup
                .defaults
                .push(format!("\n\t\t\t{}_en_reg <= (others => '0');\n\t\t\t{}_en <= '0';", cur.name, cur.name));
            setup.pop.push(format!("\n\t\t\t\t\t{}_en <= {}_en_reg(size-1);", cur.name, cur.name));
            setup.push.push(format!(
                "\n\t\t\t\t\t{}_en_reg <= {}_en_reg({}_en_reg'high - 1 downto 0) & {}_en_in;",
                cur.name, cur.name, cur.name, cur.name
            ));
            setup.final_mapping.push(format!("\n\t{}_en_out <= {}_en;", cur.name, cur.name));
        });

        setup
    }
}

#[cfg(test)]
mod queue_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_queue_file() {
        let example_file_content = "input a : Bool\ninput b : Int8\noutput c := a\noutput d := b + 7";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = VHDLQueue::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/queue/*");
        VHDLGenerator::generate_and_create(&input_manager, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn test_queue() {
        let example_file_content = "input a : Bool\ninput b : Int8\noutput c := a\noutput d := b + 7";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let input_manager = VHDLQueue::new(&lola_instance);
        let tera: Tera = compile_templates!("templates/queue/*");
        let result = VHDLGenerator::generate(&input_manager, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result_entity = &result[first_pos..last_pos];
        let result_entity: Vec<&str> = result_entity.split("\n").collect();
        //check lines
        assert_eq!(result_entity[5].trim(), "a_data_in : in std_logic;");
        assert_eq!(result_entity[6].trim(), "a_en_in : in std_logic;");
        assert_eq!(result_entity[9].trim(), "c_en_in : in std_logic;");
        assert_eq!(result_entity[10].trim(), "d_en_in : in std_logic;");
        //take signal
        let first_pos = result.find("architecture").expect("expected entity declaration");
        let last_pos = result.find("begin").expect("expected begin process");
        let result_signal = &result[first_pos..last_pos];
        let result_signal: Vec<&str> = result_signal.split("\n").collect();
        //check lines
        assert_eq!(result_signal[5].trim(), "signal a_data_reg : bit_array(1 downto 0);");
        assert_eq!(result_signal[6].trim(), "signal a_en_reg : bit_array(1 downto 0);");
        assert_eq!(result_signal[7].trim(), "signal a_data : std_logic;");
        assert_eq!(result_signal[8].trim(), "signal a_en: std_logic;");
    }
}
