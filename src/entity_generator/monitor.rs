use crate::entity_generator::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct Monitor<'a> {
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> Monitor<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir) -> Monitor<'a> {
        Monitor { ir }
    }
}

impl GenerateVhdlCode for Monitor<'_> {
    fn template_name(&self) -> String {
        "monitor.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "monitor.vhdl".to_string()
    }
}

impl Serialize for Monitor<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_monitor_setup();
        let mut s = serializer.serialize_struct("Monitor", 20)?;
        s.serialize_field("input", &setup.input.concat())?;
        s.serialize_field("output", &setup.output.concat())?;
        s.serialize_field("hlc_component_input", &setup.hlc_component_input.concat())?;
        s.serialize_field("component_in", &setup.component_in.concat())?;
        s.serialize_field("component_out", &setup.component_out.concat())?;
        s.serialize_field("evaluator_component_input", &setup.evaluator_component_input.concat())?;
        s.serialize_field("evaluator_component_output", &setup.evaluator_component_output.concat())?;
        s.serialize_field("timing_manager_signals", &setup.timing_manager_signals.concat())?;
        s.serialize_field("queue_signals", &setup.queue_signals.concat())?;
        s.serialize_field("evaluator_signals", &setup.evaluator_signals.concat())?;
        s.serialize_field("monitor_signals", &setup.monitor_signals.concat())?;
        s.serialize_field("hlc_instance_input", &setup.hlc_instance_input.concat())?;
        s.serialize_field("hlc_instance_output", &setup.hlc_instance_output.concat())?;
        s.serialize_field("queue_instance_input", &setup.queue_instance_input.concat())?;
        s.serialize_field("queue_instance_output", &setup.queue_instance_output.concat())?;
        s.serialize_field("evaluator_instance_input", &setup.evaluator_instance_input.concat())?;
        s.serialize_field("evaluator_instance_output", &setup.evaluator_instance_output.concat())?;
        s.serialize_field("signal_default_values", &setup.signal_default_values.concat())?;
        s.serialize_field("signal_assignment", &setup.signal_assignment.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;

        s.end()
    }
}

pub(crate) struct MonitorSetup {
    input: Vec<String>,
    output: Vec<String>,
    hlc_component_input: Vec<String>,
    component_in: Vec<String>,
    component_out: Vec<String>,
    evaluator_component_input: Vec<String>,
    evaluator_component_output: Vec<String>,
    timing_manager_signals: Vec<String>,
    queue_signals: Vec<String>,
    evaluator_signals: Vec<String>,
    monitor_signals: Vec<String>,
    hlc_instance_input: Vec<String>,
    hlc_instance_output: Vec<String>,
    queue_instance_input: Vec<String>,
    queue_instance_output: Vec<String>,
    evaluator_instance_input: Vec<String>,
    evaluator_instance_output: Vec<String>,
    signal_default_values: Vec<String>,
    signal_assignment: Vec<String>,
    final_mapping: Vec<String>,
}

impl MonitorSetup {
    fn new() -> MonitorSetup {
        MonitorSetup {
            input: Vec::new(),
            output: Vec::new(),
            hlc_component_input: Vec::new(),
            component_in: Vec::new(),
            component_out: Vec::new(),
            evaluator_component_input: Vec::new(),
            evaluator_component_output: Vec::new(),
            timing_manager_signals: Vec::new(),
            queue_signals: Vec::new(),
            evaluator_signals: Vec::new(),
            monitor_signals: Vec::new(),
            hlc_instance_input: Vec::new(),
            hlc_instance_output: Vec::new(),
            queue_instance_input: Vec::new(),
            queue_instance_output: Vec::new(),
            evaluator_instance_input: Vec::new(),
            evaluator_instance_output: Vec::new(),
            signal_default_values: Vec::new(),
            signal_assignment: Vec::new(),
            final_mapping: Vec::new(),
        }
    }
}

impl Monitor<'_> {
    fn generate_monitor_setup(&self) -> MonitorSetup {
        let mut setup = MonitorSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                let vhdl_type = get_vhdl_type(&cur.ty);
                let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
                setup.input.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_data_in_new_input : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.output.push(format!("\n\t\t{}_stream: out {};", cur.name, vhdl_init_type));
                setup.hlc_component_input.push(format!(
                    "\n\t\t\t{}_data_in : in {};\n\t\t\t{}_push_in : std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup.component_in.push(format!(
                    "\n\t\t\t{}_data_in : in {};\n\t\t\t{}_en_in : in std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.component_out.push(format!(
                    "\n\t\t\t{}_data_out : out {};\n\t\t\t{}_en_out : out std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup
                    .evaluator_component_input
                    .push(format!("\n\t\t\t{} : in {};\n\t\t\t{}_en : in std_logic;", cur.name, vhdl_type, cur.name));
                setup.timing_manager_signals.push(format!(
                    "\n\tsignal {}_data_timing : {};\n\tsignal {}_en_timing : std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.queue_signals.push(format!(
                    "\n\tsignal {}_data_queue : {};\n\tsignal {}_en_queue : std_logic;",
                    cur.name, vhdl_type, cur.name
                ));
                setup.monitor_signals.push(format!("\n\tsignal {}_stream_reg : {};", cur.name, vhdl_init_type));
                setup.hlc_instance_input.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_in,\n\t\t\t{}_push_in => {}_data_in_new_input,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.hlc_instance_output.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_timing,\n\t\t\t{}_en_out => {}_en_timing,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.queue_instance_input.push(format!(
                    "\n\t\t\t{}_data_in => {}_data_timing,\n\t\t\t{}_en_in => {}_en_timing,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.queue_instance_output.push(format!(
                    "\n\t\t\t{}_data_out => {}_data_queue,\n\t\t\t{}_en_out => {}_en_queue,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.evaluator_instance_input.push(format!(
                    "\n\t\t\t{} => {}_data_queue,\n\t\t\t{}_en => {}_en_queue,",
                    cur.name, cur.name, cur.name, cur.name
                ));
                setup.signal_default_values.push(format!(
                    "\n\t\t\t{}_stream_reg <= {};",
                    cur.name,
                    generate_vhdl_type_default_initialisation(&cur.ty)
                ));
                setup.signal_assignment.push(format!(
                    "\n\t\t\t{}_stream_reg <= {};",
                    cur.name,
                    get_vhdl_initial_type_cast(&cur.ty, format!("{}_data_queue", cur.name))
                ));
                setup.final_mapping.push(format!("\n\t{}_stream <= {}_stream_reg;", cur.name, cur.name));
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            let vhdl_type = get_vhdl_type(&cur.ty);
            let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
            setup.output.push(format!("\n\t\t{}_stream: out {};", cur.name, vhdl_init_type));
            setup.component_out.push(format!("\n\t\t\t{}_en_out : out std_logic;", cur.name));
            setup.component_in.push(format!("\n\t\t\t{}_en_in : in std_logic;", cur.name));
            setup.evaluator_component_input.push(format!("\n\t\t\t{}_en : in std_logic;", cur.name));
            setup.evaluator_component_output.push(format!("\n\t\t\t{} : out {};", cur.name, vhdl_type));
            setup.timing_manager_signals.push(format!("\n\tsignal {}_en_timing : std_logic;", cur.name));
            setup.queue_signals.push(format!("\n\tsignal {}_en_queue : std_logic;", cur.name));
            setup.evaluator_signals.push(format!("\n\tsignal {}_stream_evaluator : {};", cur.name, vhdl_type));
            setup.monitor_signals.push(format!("\n\tsignal {}_stream_reg : {};", cur.name, vhdl_init_type));
            setup.hlc_instance_output.push(format!("\n\t\t\t{}_en_out => {}_en_timing,", cur.name, cur.name));
            setup.queue_instance_input.push(format!("\n\t\t\t{}_en_in => {}_en_timing,", cur.name, cur.name));
            setup.queue_instance_output.push(format!("\n\t\t\t{}_en_out => {}_en_queue,", cur.name, cur.name));
            setup.evaluator_instance_input.push(format!("\n\t\t\t{}_en => {}_en_queue,", cur.name, cur.name));
            setup.evaluator_instance_output.push(format!("\n\t\t\t{} => {}_stream_evaluator,", cur.name, cur.name));
            setup.signal_default_values.push(format!(
                "\n\t\t\t{}_stream_reg <= {};",
                cur.name,
                generate_vhdl_type_default_initialisation(&cur.ty)
            ));
            setup.signal_assignment.push(format!(
                "\n\t\t\t{}_stream_reg <= {};",
                cur.name,
                get_vhdl_initial_type_cast(&cur.ty, format!("{}_stream_evaluator", cur.name))
            ));
            setup.final_mapping.push(format!("\n\t{}_stream <= {}_stream_reg;", cur.name, cur.name));
        });
        setup
    }
}

#[cfg(test)]
mod monitor_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_monitor_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap();
        let monitor = Monitor::new(&lola_instance);
        let tera: Tera = tera::compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&monitor, &tera, &PathBuf::from("target/test_files"))
    }
}
