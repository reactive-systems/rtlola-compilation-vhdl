use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowGeneral;
use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowTrait;
use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::ir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct SlidingWindowAvgVHDL<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) ir: &'a RTLolaIR,
    pub(crate) num_buckets: u16,
}

impl<'a> SlidingWindowAvgVHDL<'a> {
    pub(crate) fn new(
        sliding_window: &'a SlidingWindow,
        ir: &'a RTLolaIR,
        num_buckets: u16,
    ) -> SlidingWindowAvgVHDL<'a> {
        SlidingWindowAvgVHDL { sliding_window, ir, num_buckets }
    }
}

impl<'a> SlidingWindowTrait for SlidingWindowAvgVHDL<'a> {
    fn sw_data_buckets(&self) -> String {
        let array_ty = generate_vhdl_array_type_downwards(&self.sliding_window.ty, self.num_buckets - 1);
        format!("signal count_buckets : {};\n\tsignal sum_buckets : {};", array_ty, array_ty)
    }

    fn temporary_variables(&self) -> String {
        String::new()
    }

    fn valid_upd(&self) -> String {
        "'0'".to_string()
    }

    fn set_sw_buckets_to_default(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        let count_default = format!("count_buckets(count_buckets'high downto 0) <= (others => {});", ty_default);
        let sum_default = format!("sum_buckets(sum_buckets'high downto 0) <= (others => {});", ty_default);
        format!("{}\n\t\t\t{}", count_default, sum_default)
    }

    fn create_and_shift_sw_buckets(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        let count = format!("count_buckets <= count_buckets(count_buckets'high-1 downto 0) & {};", ty_default);
        let sum = format!("sum_buckets <= sum_buckets(sum_buckets'high-1 downto 0) & {};", ty_default);
        format!("{}\n\t\t\t\t\t{}", count, sum)
    }

    fn map_and_update_last_sw_bucket(&self) -> String {
        let data_valid = "data_valid_buckets(0) <= '1';".to_string();
        let count = format!(
            "count_buckets(0) <= {}",
            format!("count_buckets(0) + {};", get_count_upd(&self.sliding_window.ty, "count_buckets(0)")),
        );
        let sum = "sum_buckets(0) <= sum_buckets(0) + data_in;".to_string();
        format!("{}\n\t\t\t\t{}\n\t\t\t\t{}", data_valid, count, sum)
    }

    fn finalize_sw(&self) -> String {
        let mut fin_sum = "sum_buckets(0)".to_string();
        for i in 1..self.num_buckets {
            fin_sum = format!("{} + sum_buckets({})", fin_sum, i);
        }

        let mut fin_count = "count_buckets(0)".to_string();
        for i in 1..self.num_buckets {
            fin_count = format!("{} + count_buckets({})", fin_count, i);
        }

        let div = format!("(({}) / ({}));", fin_sum, fin_count);
        format!("sw_data <= {}", div)
    }

    fn finalize_valid(&self) -> String {
        let mut fin_val = "data_valid_buckets(0)".to_string();
        for i in 1..self.num_buckets {
            fin_val = format!("{} or data_valid_buckets({})", fin_val, i);
        }
        fin_val
    }
}

#[cfg(test)]
mod sliding_window_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use rtlola_frontend::TypeConfig;
    use std::path::PathBuf;
    use tera::Tera;

    fn parse(spec: &str) -> Result<RTLolaIR, String> {
        rtlola_frontend::parse("stdin", spec, crate::CONFIG)
    }

    #[test]
    fn sliding_window_stream_test() {
        let example_file_content = "input a : Int8\noutput b @ 2Hz := a.aggregate(over: 1s, using: avg).defaults(to:0)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let tera: Tera = compile_templates!("templates/low_level_controller/sliding_windows/*");
        VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_avg(sw, &lola_instance, 4, 4),
            &tera,
            &PathBuf::from("target/test_files"),
        );
    }
}
