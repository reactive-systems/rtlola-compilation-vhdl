use crate::entity_generator::*;
use rtlola_frontend::RtLolaMir;

pub(crate) mod check_new_input;
pub(crate) mod event_delay;
pub(crate) mod event_scheduler_offline;
pub(crate) mod ext_interface;
pub(crate) mod hl_qinterface;
pub(crate) mod scheduler;
pub(crate) mod time_unit_offline;
pub(crate) mod time_unit_online;

use crate::vhdl_wrapper::type_serialize::{get_vhdl_initial_type, get_vhdl_type};
use crate::Config;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) fn generate_timing_manager_entities(config: &Config) {
    let mut target = config.target.clone();
    target.push("hlc/");
    let tera_files = config.templates.clone() + "/high_level_controller/*";
    let tera = tera::compile_templates!(&tera_files);
    let schedule = RtLolaMir::compute_schedule(&config.ir).unwrap_or_else(|e| panic!("{}", e));
    VHDLGenerator::generate_and_create(&ext_interface::ExtInterface::new(&config.ir), &tera, &target);
    VHDLGenerator::generate_and_create(&check_new_input::CheckNewInput::new(), &tera, &target);
    VHDLGenerator::generate_and_create(&event_delay::EventDelay::new(&config.ir), &tera, &target);
    if config.mode {
        VHDLGenerator::generate_and_create(&time_unit_online::TimeUnitOnline::new(), &tera, &target);
    } else {
        VHDLGenerator::generate_and_create(&time_unit_offline::TimeUnitOffline::new(), &tera, &target);
        VHDLGenerator::generate_and_create(
            &event_scheduler_offline::EventSchedulerOffline::new(&config.ir),
            &tera,
            &target,
        );
    }

    VHDLGenerator::generate_and_create(&scheduler::Scheduler::new(&schedule, &config.ir), &tera, &target);
    VHDLGenerator::generate_and_create(&hl_qinterface::HlQInterface::new(&schedule, &config.ir), &tera, &target);
    VHDLGenerator::generate_and_create(&HLC::new(&config.ir, config.mode), &tera, &target);
}

#[allow(clippy::upper_case_acronyms)]
pub(crate) struct HLC<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) online: bool,
}

impl<'a> HLC<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, online: bool) -> HLC<'a> {
        HLC { ir, online }
    }
}

impl GenerateVhdlCode for HLC<'_> {
    fn template_name(&self) -> String {
        "high_level_controller.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "high_level_controller.vhdl".to_string()
    }
}

impl Serialize for HLC<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_timing_manager_setup();
        let mut s = serializer.serialize_struct("TimingManager", 11)?;
        s.serialize_field("online", &self.online)?;
        s.serialize_field("inputs_bit_type", &setup.inputs_bit_type.concat())?;
        s.serialize_field("inputs_numeric_type", &setup.inputs_numeric_type.concat())?;
        s.serialize_field("inputs_in_en_type", &setup.inputs_in_en_type.concat())?;
        s.serialize_field("outputs_numeric_type", &setup.outputs_numeric_type.concat())?;
        s.serialize_field("input_and_outputs_en_plus_data", &setup.input_and_outputs_en_plus_data.concat())?;
        s.serialize_field("wire_signals", &setup.wire_signals.concat())?;
        s.serialize_field("input_ext_interface_instance", &setup.input_ext_interface_instance.concat())?;
        s.serialize_field("output_ext_interface_instance", &setup.output_ext_interface_instance.concat())?;
        s.serialize_field("input_event_delay_instance", &setup.input_event_delay_instance.concat())?;
        s.serialize_field("output_event_delay_instance", &setup.output_event_delay_instance.concat())?;
        s.serialize_field("input_event_scheduler_instance", &setup.input_event_scheduler_instance.concat())?;
        s.serialize_field("output_event_scheduler_instance", &setup.output_event_scheduler_instance.concat())?;
        s.serialize_field("input_hl_qinterface_instance", &setup.input_hl_qinterface_instance.concat())?;
        s.serialize_field("output_hl_qinterface_instance", &setup.output_hl_qinterface_instance.concat())?;
        s.end()
    }
}

pub(crate) struct TimingManagerSetup {
    pub(crate) inputs_bit_type: Vec<String>,
    pub(crate) inputs_numeric_type: Vec<String>,
    pub(crate) inputs_in_en_type: Vec<String>,
    pub(crate) outputs_numeric_type: Vec<String>,
    pub(crate) input_and_outputs_en_plus_data: Vec<String>,
    pub(crate) wire_signals: Vec<String>,
    pub(crate) input_ext_interface_instance: Vec<String>,
    pub(crate) output_ext_interface_instance: Vec<String>,
    pub(crate) input_event_delay_instance: Vec<String>,
    pub(crate) output_event_delay_instance: Vec<String>,
    pub(crate) input_event_scheduler_instance: Vec<String>,
    pub(crate) output_event_scheduler_instance: Vec<String>,
    pub(crate) input_hl_qinterface_instance: Vec<String>,
    pub(crate) output_hl_qinterface_instance: Vec<String>,
}

impl TimingManagerSetup {
    fn new() -> TimingManagerSetup {
        TimingManagerSetup {
            inputs_bit_type: Vec::new(),
            inputs_numeric_type: Vec::new(),
            outputs_numeric_type: Vec::new(),
            inputs_in_en_type: Vec::new(),
            input_and_outputs_en_plus_data: Vec::new(),
            wire_signals: Vec::new(),
            input_ext_interface_instance: Vec::new(),
            output_ext_interface_instance: Vec::new(),
            input_event_delay_instance: Vec::new(),
            output_event_delay_instance: Vec::new(),
            input_event_scheduler_instance: Vec::new(),
            output_event_scheduler_instance: Vec::new(),
            input_hl_qinterface_instance: Vec::new(),
            output_hl_qinterface_instance: Vec::new(),
        }
    }
}

impl HLC<'_> {
    fn generate_timing_manager_setup(&self) -> TimingManagerSetup {
        let mut setup = TimingManagerSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                let vhdl_type = get_vhdl_type(&cur.ty);
                let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
                setup.inputs_bit_type.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_push_in : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.inputs_numeric_type.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_push_in : in std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.inputs_in_en_type.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_en_in : in std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.outputs_numeric_type.push(format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_push_out : out std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.input_and_outputs_en_plus_data.push(format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_en_out : out std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.wire_signals.push(format!(
                    "\n\tsignal {}_data_from_extInterface : {};\n\tsignal {}_push_from_extInterface : std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.wire_signals.push(format!(
                    "\n\tsignal {}_data_from_eventDelay : {};\n\tsignal {}_push_from_eventDelay : std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.wire_signals.push(format!(
                    "\n\tsignal {}_data_from_eventScheduler : {};\n\tsignal {}_push_from_eventScheduler : std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.input_ext_interface_instance.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_in,\n\t\t\t{}_push_in => {}_push_in,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.output_ext_interface_instance.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_from_extInterface,\n\t\t\t{}_push_out => {}_push_from_extInterface,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.input_event_delay_instance.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_from_extInterface,\n\t\t\t{}_push_in => {}_push_from_extInterface,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.output_event_delay_instance.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_from_eventDelay,\n\t\t\t{}_push_out => {}_push_from_eventDelay,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.input_event_scheduler_instance.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_from_eventDelay,\n\t\t\t{}_push_in => {}_push_from_eventDelay,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.output_event_scheduler_instance.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_from_eventScheduler,\n\t\t\t{}_push_out => {}_push_from_eventScheduler,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                if self.online {
                    setup.input_hl_qinterface_instance.push(format!(
                        "\n\t\t\t{}_data_in => {}_data_from_eventDelay,\n\t\t\t{}_en_in => {}_push_from_eventDelay,",
                        cur.name, cur.name, cur.name, cur.name
                    ));

                } else {
                    setup.input_hl_qinterface_instance.push(format!(
                        "\n\t\t\t{}_data_in => {}_data_from_eventScheduler,\n\t\t\t{}_en_in => {}_push_from_eventScheduler,",
                        cur.name, cur.name, cur.name, cur.name
                    ));
                }
                setup.output_hl_qinterface_instance.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_out,\n\t\t\t{}_en_out => {}_en_out,",
                    cur.name, cur.name, cur.name, cur.name
                ));
            }
        });

        self.ir.outputs.iter().for_each(|cur| {
            setup.input_and_outputs_en_plus_data.push(format!("\n\t\t{}_en_out : out std_logic;", cur.name));
            setup.output_hl_qinterface_instance.push(format!("\n\t\t\t{}_en_out => {}_en_out,", cur.name, cur.name));
        });

        setup
    }
}

#[cfg(test)]
mod timing_manager_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_timing_manager_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let timing_manager = HLC::new(&lola_instance, false);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&timing_manager, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn timing_manager_test() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let timing_manager = HLC::new(&lola_instance, true);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&timing_manager, &tera);
        //take signal
        let first_pos = result.find("-- Internal Signal Declarations").expect("expected Signal Declaration");
        let last_pos = result.find("signal slow_hlc_clk : std_logic;").expect("expected next deadline signal");
        let result_registers = &result[first_pos..last_pos];
        let result_registers: Vec<&str> = result_registers.split("\n").collect();
        //check lines
        assert_eq!(result_registers[10].trim(), "signal a_data_from_extInterface : signed(7 downto 0);");
        assert_eq!(result_registers[11].trim(), "signal a_push_from_extInterface : std_logic;");
        assert_eq!(result_registers[12].trim(), "signal a_data_from_eventDelay : signed(7 downto 0);");
        assert_eq!(result_registers[13].trim(), "signal a_push_from_eventDelay : std_logic;");
        assert_eq!(result_registers[14].trim(), "signal a_data_from_eventScheduler : signed(7 downto 0);");
        assert_eq!(result_registers[15].trim(), "signal a_push_from_eventScheduler : std_logic;");
    }
}
