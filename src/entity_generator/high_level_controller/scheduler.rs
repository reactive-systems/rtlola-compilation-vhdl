use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use rtlola_frontend::mir::Schedule;

pub(crate) struct Scheduler<'a> {
    pub(crate) schedule: &'a Schedule,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> Scheduler<'a> {
    pub(crate) fn new(schedule: &'a Schedule, ir: &'a RtLolaMir) -> Scheduler<'a> {
        Scheduler { schedule, ir }
    }
}

impl GenerateVhdlCode for Scheduler<'_> {
    fn template_name(&self) -> String {
        "scheduler.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "scheduler.vhdl".to_string()
    }
}

impl Serialize for Scheduler<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = &self.generate_scheduler_setup();
        let mut s = serializer.serialize_struct("PeriodicManager", 7)?;
        s.serialize_field("size_dls_minus_1", &(self.schedule.deadlines.len() - 1))?;
        s.serialize_field("size_dls", &self.schedule.deadlines.len())?;
        //TODO Problem u128 not supported by tera -> any other idea?
        s.serialize_field("initial_deadline", &(self.schedule.deadlines[0].pause.as_nanos() as u64))?;
        s.serialize_field("deadlines", &setup.deadlines.concat())?;
        s.serialize_field("print_periodic_streams", &setup.print_periodic_streams.concat())?;
        s.serialize_field("print_hyper_period", &setup.print_hyper_period)?;
        s.serialize_field("print_offset_array", &setup.print_offset_array.concat())?;
        s.end()
    }
}

pub(crate) struct SchedulerSetup {
    pub(crate) deadlines: Vec<String>,
    pub(crate) print_periodic_streams: Vec<String>,
    pub(crate) print_hyper_period: String,
    pub(crate) print_offset_array: Vec<String>,
}

impl SchedulerSetup {
    fn new() -> SchedulerSetup {
        SchedulerSetup {
            deadlines: Vec::new(),
            print_periodic_streams: Vec::new(),
            print_hyper_period: String::new(),
            print_offset_array: Vec::new(),
        }
    }
}

impl Scheduler<'_> {
    fn generate_scheduler_setup(&self) -> SchedulerSetup {
        let mut setup = SchedulerSetup::new();
        let mut counter = -1;
        setup.print_hyper_period = format!("{} seconds", self.schedule.hyper_period.unwrap().as_secs_f64());
        setup.print_offset_array.push("|".to_string());
        self.schedule.deadlines.iter().for_each(|cur| {
            counter += 1;
            setup.deadlines.push(format!(
                "\n\t\t\toffset_per_deadline({}) <= to_unsigned({}, offset_per_deadline({})'length);",
                counter,
                cur.pause.as_nanos(),
                counter
            ));
            setup.print_offset_array.push(format!("| {} ", cur.pause.as_secs_f64()));
        });
        setup.print_offset_array.push("||".to_string());
        self.ir.time_driven.iter().for_each(|cur| {
            let name = self.ir.get_name_for_stream_ref(cur.reference);
            let freq = cur.frequency.value;
            setup.print_periodic_streams.push(format!("\n--* - {} @ {}Hz *--", name, freq));
        });
        setup
    }
}

#[cfg(test)]
mod periodic_manager_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_periodic_manager_file() {
        let example_file_content = "input a : Int8\noutput b @1Hz := a.hold().defaults(to:0) + 3\noutput c @2Hz := a.hold().defaults(to:0) + 6";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &RtLolaMir::compute_schedule(&lola_instance).unwrap_or_else(|e| panic!("{}", e));
        let periodic_manager = Scheduler::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&periodic_manager, &tera, &PathBuf::from("target/test_files"))
    }

    #[test]
    fn periodic_manager_test() {
        let example_file_content = "input a : Int8\noutput b @1Hz := a.hold().defaults(to:0) + 3\noutput c @2Hz := a.hold().defaults(to:0) + 6";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &RtLolaMir::compute_schedule(&lola_instance).unwrap_or_else(|e| panic!("{}", e));
        let periodic_manager = Scheduler::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&periodic_manager, &tera);
        //take signal
        let first_pos = result.find("architecture").expect("expected entity declaration");
        let last_pos = result.find("begin").expect("expected begin process");
        let result_signal = &result[first_pos..last_pos];
        let result_signal: Vec<&str> = result_signal.split("\n").collect();
        //check lines
        assert_eq!(result_signal[4].trim(), "signal offset_per_deadline : unsigned64_array(1 downto 0);");
        //take deadlines
        let first_pos = result.find("if (rst = '1') then").expect("expected rst handling");
        let last_pos = result.find("elsif (rising_edge(clk)) then").expect("expected deadline handling");
        let result_rst = &result[first_pos..last_pos];
        let result_rst: Vec<&str> = result_rst.split("\n").collect();
        //check lines
        assert_eq!(
            result_rst[2].trim(),
            "time_of_next_deadline <= to_unsigned(500000000, time_of_next_deadline'length);"
        );
        assert_eq!(result_rst[3].trim(), "last_deadline_id <= 0;");
        assert_eq!(
            result_rst[9].trim(),
            "offset_per_deadline(0) <= to_unsigned(500000000, offset_per_deadline(0)'length);"
        );
        assert_eq!(
            result_rst[10].trim(),
            "offset_per_deadline(1) <= to_unsigned(500000000, offset_per_deadline(1)'length);"
        );
    }
}
