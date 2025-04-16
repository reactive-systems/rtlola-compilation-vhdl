use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use rtlola_frontend::{
    mir::{ActivationCondition, Schedule, StreamReference},
    RtLolaMir,
};

pub(crate) struct HlQInterface<'a> {
    pub(crate) schedule: &'a Schedule,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> HlQInterface<'a> {
    pub(crate) fn new(schedule: &'a Schedule, ir: &'a RtLolaMir) -> HlQInterface<'a> {
        let mut event = Vec::new();
        let mut time = Vec::new();
        ir.outputs.iter().for_each(|cur| {
            if ir.event_driven.iter().any(|cur_event| cur_event.reference == cur.reference) {
                event.push(cur);
            } else {
                time.push(cur);
            }
        });
        HlQInterface { schedule, ir }
        //        HlQInterface { schedule, ir, event_driven_output: event, time_driven_output: time }
    }
}

impl GenerateVhdlCode for HlQInterface<'_> {
    fn template_name(&self) -> String {
        "hl_qinterface.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "hl_qinterface.vhdl".to_string()
    }
}

impl Serialize for HlQInterface<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = &self.generate_coordinator_setup();
        let mut s = serializer.serialize_struct("Coordinator", 17)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("push_to_queue_signals", &setup.push_to_queue_signals.concat())?;
        s.serialize_field("default_assignments", &setup.default_assignments.concat())?;
        s.serialize_field("init_dl_arrays", &setup.init_dl_arrays.concat())?;
        s.serialize_field("deadline_len", &(self.schedule.deadlines.len()))?;
        s.serialize_field("deadline_handling", &setup.deadline_handling.concat())?;
        s.serialize_field("event_handling", &setup.event_handling.concat())?;
        s.serialize_field("en_no_stream", &setup.en_no_stream.concat())?;
        s.serialize_field("final_mapping", &setup.final_mapping.concat())?;
        let print_input_streams_without_offset: Vec<String> =
            setup.print_input_streams.iter().map(|cur| format!("\n--* {}", cur)).collect();
        let print_input_streams_with_offset: Vec<String> =
            setup.print_input_streams.iter().map(|cur| format!("\n\t\t\t\t--* {}", cur)).collect();
        s.serialize_field("print_input_streams_without_offset", &print_input_streams_without_offset.concat())?;
        s.serialize_field("print_input_streams_with_offset", &print_input_streams_with_offset.concat())?;
        let print_event_output_streams_without_offset: Vec<String> =
            setup.print_event_output_streams.iter().map(|cur| format!("\n--* {}", cur)).collect();
        let print_event_output_streams_with_offset: Vec<String> =
            setup.print_event_output_streams.iter().map(|cur| format!("\n\t\t\t\t--* {}", cur)).collect();
        s.serialize_field(
            "print_event_output_streams_without_offset",
            &print_event_output_streams_without_offset.concat(),
        )?;
        s.serialize_field("print_event_output_streams_with_offset", &print_event_output_streams_with_offset.concat())?;
        let print_periodic_output_streams_without_offset: Vec<String> =
            setup.print_periodic_output_streams.iter().map(|cur| format!("\n--* {}", cur)).collect();
        let print_periodic_output_streams_with_offset: Vec<String> =
            setup.print_periodic_output_streams.iter().map(|cur| format!("\n\t\t\t\t--* {}", cur)).collect();
        s.serialize_field(
            "print_periodic_output_streams_without_offset",
            &print_periodic_output_streams_without_offset.concat(),
        )?;
        s.serialize_field(
            "print_periodic_output_streams_with_offset",
            &print_periodic_output_streams_with_offset.concat(),
        )?;
        s.serialize_field("print_deadline_array", &format!("||{}|", setup.print_deadline_array.concat()))?;
        s.end()
    }
}

pub(crate) struct HLQInterfaceSetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) push_to_queue_signals: Vec<String>,
    pub(crate) default_assignments: Vec<String>,
    pub(crate) init_dl_arrays: Vec<String>,
    pub(crate) deadline_handling: Vec<String>,
    pub(crate) event_handling: Vec<String>,
    pub(crate) en_no_stream: Vec<String>,
    pub(crate) final_mapping: Vec<String>,
    pub(crate) print_input_streams: Vec<String>,
    pub(crate) print_event_output_streams: Vec<String>,
    pub(crate) print_periodic_output_streams: Vec<String>,
    pub(crate) print_deadline_array: Vec<String>,
}

impl HLQInterfaceSetup {
    fn new() -> HLQInterfaceSetup {
        HLQInterfaceSetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            push_to_queue_signals: Vec::new(),
            default_assignments: Vec::new(),
            init_dl_arrays: Vec::new(),
            deadline_handling: Vec::new(),
            event_handling: Vec::new(),
            en_no_stream: Vec::new(),
            final_mapping: Vec::new(),
            print_input_streams: Vec::new(),
            print_periodic_output_streams: Vec::new(),
            print_event_output_streams: Vec::new(),
            print_deadline_array: Vec::new(),
        }
    }
}

impl HlQInterface<'_> {
    fn generate_coordinator_setup(&self) -> HLQInterfaceSetup {
        let mut setup = HLQInterfaceSetup::new();
        self.ir.inputs.iter().for_each(|cur| {
            let stream_with_ac_as_string = format!("{} @ {{ {} }}", cur.name, cur.name);
            if cur.name.as_str() != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_en_in : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.outputs.push(format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_en_out : out std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.push_to_queue_signals.push(format!(
                    "\n\tsignal {}_data_push : {};\n\tsignal {}_en_push: std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.default_assignments.push(format!(
                    "\n\t\t\t{}_data_push <= {};\n\t\t\t{}_en_push <= '0';",
                    cur.name,
                    generate_vhdl_type_default_initialisation(&cur.ty),
                    cur.name
                ));
                setup
                    .deadline_handling
                    .push(format!("\n\t\t\t\t--* {}\n\t\t\t\t{}_en_push <= '0';", stream_with_ac_as_string, cur.name));
                setup.event_handling.push(format!(
                    "\n\t\t\t\t--* {}\n\t\t\t\t{}_data_push <= {}_data_in;\n\t\t\t\t{}_en_push <= {}_en_in;",
                    stream_with_ac_as_string, cur.name, cur.name, cur.name, cur.name
                ));
                setup.en_no_stream.push(format!("\n\t\t\t\t{}_en_push <= '0';", cur.name));
                setup.final_mapping.push(format!(
                    "\n\t{}_data_out <= {}_data_push;\n\t{}_en_out <= {}_en_push;",
                    cur.name, cur.name, cur.name, cur.name
                ));
            }
            setup.print_input_streams.push(format!("- {}", stream_with_ac_as_string));
        });
        self.ir.outputs.iter().for_each(|cur| {
            setup.outputs.push(format!("\n\t\t{}_en_out : out std_logic;", cur.name));
            setup.push_to_queue_signals.push(format!("\n\tsignal {}_en_push : std_logic;", cur.name));
            setup.default_assignments.push(format!("\n\t\t\t{}_en_push <= '0';", cur.name));
            setup.en_no_stream.push(format!("\n\t\t\t\t{}_en_push <= '0';", cur.name));
            setup.final_mapping.push(format!("\n\t{}_en_out <= {}_en_push;", cur.name, cur.name));
        });
        let mut counter = -1;
        self.schedule.deadlines.iter().for_each(|cur_deadline| {
            counter += 1;
            let deadline_array_vhdl: Vec<String> = self
                .ir
                .time_driven
                .iter()
                .map(|cur_out| {
                    let cur_stream_name = self.ir.stream(cur_out.reference).name();
                    format!(
                        "\n\t\t\t{}_en_array({}) <= '{}';",
                        cur_stream_name,
                        counter,
                        if cur_deadline.due.contains(&rtlola_frontend::mir::Task::Evaluate(cur_out.reference.out_ix()))
                        {
                            "1"
                        } else {
                            "0"
                        }
                    )
                })
                .collect();
            let mut first = true;
            let deadline_array_string: Vec<String> = cur_deadline
                .due
                .iter()
                .map(|cur_periodic_stream_in_deadline| {
                    let outref = match cur_periodic_stream_in_deadline {
                        rtlola_frontend::mir::Task::Evaluate(r) => r,
                        rtlola_frontend::mir::Task::Spawn(r) => r,
                        rtlola_frontend::mir::Task::Close(r) => r,
                    };
                    let res = format!(
                        "{}{}",
                        if first { " " } else { ", " },
                        self.ir.stream(StreamReference::Out(*outref)).name()
                    );
                    first = false;
                    res
                })
                .collect();
            setup.print_deadline_array.push(format!("{} |", deadline_array_string.concat()));
            setup.init_dl_arrays.push(deadline_array_vhdl.concat());
        });
        self.ir.event_driven.iter().for_each(|cur| {
            let mut input_dependencies = Vec::new();
            let mut ac_as_strings: Vec<String> = Vec::new();
            let mut first = true;
            let cur_output_stream = self.ir.output(cur.reference);
            //TODO activation condition with disjunction
            let input_streams_for_conjunction = HlQInterface::generate_activation_condition(&cur.ac);
            input_streams_for_conjunction.iter().for_each(|cur_dep| {
                let input_dep_name = self.ir.stream(**cur_dep).name();
                if input_dep_name != "time" {
                    input_dependencies.push(format!(" and {}_en_in", input_dep_name))
                }
                if first {
                    first = false;
                    ac_as_strings.push(input_dep_name.to_string());
                } else {
                    ac_as_strings.push(format!(", {}", input_dep_name));
                }
            });
            let stream_with_ac_as_string = format!("{} @ {{ {} }}", cur_output_stream.name, ac_as_strings.concat());
            setup.deadline_handling.push(format!(
                "\n\t\t\t\t--* {}\n\t\t\t\t{}_en_push <= '0';",
                stream_with_ac_as_string, cur_output_stream.name
            ));
            setup.event_handling.push(format!(
                "\n\t\t\t\t--* {}\n\t\t\t\t{}_en_push <= '1'{};",
                stream_with_ac_as_string,
                cur_output_stream.name,
                input_dependencies.concat()
            ));
            setup.print_event_output_streams.push(format!("- {}", stream_with_ac_as_string));
        });
        self.ir.time_driven.iter().for_each(|cur| {
            let name = self.ir.stream(cur.reference).name();
            let freq = cur.frequency.value;
            let stream_with_ac_as_string = format!("{} @ {}Hz", name, freq);
            setup.deadline_handling.push(format!(
                "\n\t\t\t\t--* {}\n\t\t\t\t{}_en_push <= {}_en_array(last_deadline_id);",
                stream_with_ac_as_string, name, name
            ));
            setup
                .event_handling
                .push(format!("\n\t\t\t\t--* {}\n\t\t\t\t{}_en_push <= '0';", stream_with_ac_as_string, name));
            setup.push_to_queue_signals.push(format!(
                "\n\tsignal {}_en_array : bit_array({} downto 0);",
                name,
                self.schedule.deadlines.len() - 1
            ));
            setup.print_periodic_output_streams.push(format!("- {}", stream_with_ac_as_string));
        });
        setup
    }

    //TODO implement disjunction
    fn generate_activation_condition(ac: &ActivationCondition) -> Vec<&StreamReference> {
        match ac {
            ActivationCondition::True => Vec::new(),
            ActivationCondition::Disjunction(_) => unimplemented!(),
            ActivationCondition::Conjunction(args) => {
                let mut cur_ac = Vec::new();
                args.iter().for_each(|arg| {
                    let arg_ac = HlQInterface::generate_activation_condition(arg);
                    arg_ac.iter().for_each(|cur_stream| {
                        if !cur_ac.contains(cur_stream) {
                            cur_ac.push(cur_stream);
                        }
                    })
                });
                cur_ac
            }
            ActivationCondition::Stream(stream_ref) => vec![stream_ref],
        }
    }
}

#[cfg(test)]
mod coordinator_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_coordinator_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b\noutput f := e +3";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &RtLolaMir::compute_schedule(&lola_instance).unwrap_or_else(|e| panic!("{}", e));
        let eval_controller = HlQInterface::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&eval_controller, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn test_coordinator() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b\noutput f := e +3";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &RtLolaMir::compute_schedule(&lola_instance).unwrap_or_else(|e| panic!("{}", e));
        let eval_controller = HlQInterface::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&eval_controller, &tera);
        //take signal
        let first_pos = result.find("Reset Phase").expect("expected default handling");
        let last_pos = result.find("Deadline Handling").expect("expected deadline handling");
        let result_default_handling = &result[first_pos..last_pos];
        let result_default_handling: Vec<&str> = result_default_handling.split("\n").collect();
        //check lines
        assert_eq!(result_default_handling[16].trim(), "c_en_array(0) <= '0';");
        assert_eq!(result_default_handling[17].trim(), "d_en_array(0) <= '1';");
        assert_eq!(result_default_handling[18].trim(), "c_en_array(1) <= '1';");
        assert_eq!(result_default_handling[19].trim(), "d_en_array(1) <= '1';");
        //take signal
        let first_pos = result.find("Deadline Handling").expect("expected deadline handling");
        let last_pos = result.find("Event Handling").expect("expected event handling");
        let result_deadline_handling = &result[first_pos..last_pos];
        let result_deadline_handling: Vec<&str> = result_deadline_handling.split("\n").collect();
        //check lines
        assert_eq!(result_deadline_handling[5].trim(), "a_en_push <= '0';");
        assert_eq!(result_deadline_handling[7].trim(), "b_en_push <= '0';");
        assert_eq!(result_deadline_handling[9].trim(), "e_en_push <= '0';");
        assert_eq!(result_deadline_handling[13].trim(), "c_en_push <= c_en_array(last_deadline_id);");
        assert_eq!(result_deadline_handling[15].trim(), "d_en_push <= d_en_array(last_deadline_id);");
        //take signal
        let first_pos = result.find("Event Handling").expect("expected event handling");
        let last_pos = result.find("Enable No Stream").expect("expected unset push signal");
        let result_event_handling = &result[first_pos..last_pos];
        let result_event_handling: Vec<&str> = result_event_handling.split("\n").collect();
        //check lines
        assert_eq!(result_event_handling[4].trim(), "a_data_push <= a_data_in;");
        assert_eq!(result_event_handling[5].trim(), "a_en_push <= a_en_in;");
        assert_eq!(result_event_handling[7].trim(), "b_data_push <= b_data_in;");
        assert_eq!(result_event_handling[8].trim(), "b_en_push <= b_en_in;");
        assert_eq!(result_event_handling[10].trim(), "e_en_push <= '1' and a_en_in and b_en_in;");
        assert_eq!(result_event_handling[14].trim(), "c_en_push <= '0';");
        assert_eq!(result_event_handling[16].trim(), "d_en_push <= '0';");
    }
}
