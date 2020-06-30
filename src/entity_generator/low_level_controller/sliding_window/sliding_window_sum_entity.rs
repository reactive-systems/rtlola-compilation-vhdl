use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowGeneral;
use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowTrait;
use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::ir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct SlidingWindowSumVHDL<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) ir: &'a RTLolaIR,
    pub(crate) num_buckets: u16,
}

impl<'a> SlidingWindowSumVHDL<'a> {
    pub(crate) fn new(
        sliding_window: &'a SlidingWindow,
        ir: &'a RTLolaIR,
        num_buckets: u16,
    ) -> SlidingWindowSumVHDL<'a> {
        SlidingWindowSumVHDL { sliding_window, ir, num_buckets }
    }
}

impl<'a> SlidingWindowTrait for SlidingWindowSumVHDL<'a> {
    fn sw_data_buckets(&self) -> String {
        let array_ty = generate_vhdl_array_type_downwards(&self.sliding_window.ty, self.num_buckets - 1);
        format!("signal sum_buckets : {};", array_ty)
    }

    fn temporary_variables(&self) -> String {
        String::new()
    }

    fn valid_upd(&self) -> String {
        "'1'".to_string()
    }

    fn set_sw_buckets_to_default(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        format!("sum_buckets(sum_buckets'high downto 0) <= (others => {});", ty_default)
    }

    fn create_and_shift_sw_buckets(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        format!("sum_buckets <= sum_buckets(sum_buckets'high-1 downto 0) & {};", ty_default)
    }

    fn map_and_update_last_sw_bucket(&self) -> String {
        format!("sum_buckets(0) <= {}", resize_float(&self.sliding_window.ty, "sum_buckets(0) + data_in".to_string()))
    }

    fn finalize_sw(&self) -> String {
        let mut fin = "sum_buckets(0)".to_string();
        for i in 1..self.num_buckets {
            fin = format!("{} + sum_buckets({})", fin, i);
        }
        let res = resize_float(&self.sliding_window.ty, fin);
        format!("sw_data <= {}", res)
    }

    fn finalize_valid(&self) -> String {
        "'1'".to_string()
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
        let example_file_content = "input a : Int8\noutput b @ 2Hz := a.aggregate(over: 1s, using: sum)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let tera: Tera = compile_templates!("templates/low_level_controller/sliding_windows/*");
        VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_sum(sw, &lola_instance, 4, 4),
            &tera,
            &PathBuf::from("target/test_files"),
        );
    }
}
