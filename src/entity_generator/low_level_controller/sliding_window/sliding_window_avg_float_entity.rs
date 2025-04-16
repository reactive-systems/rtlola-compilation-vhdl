use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowTrait;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;

pub(crate) struct SlidingWindowAvgFloatVHDL<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) num_buckets: u32,
}

impl<'a> SlidingWindowAvgFloatVHDL<'a> {
    pub(crate) fn new(sliding_window: &'a SlidingWindow, num_buckets: u32) -> SlidingWindowAvgFloatVHDL<'a> {
        SlidingWindowAvgFloatVHDL { sliding_window, num_buckets }
    }
}

impl SlidingWindowTrait for SlidingWindowAvgFloatVHDL<'_> {
    fn sw_data_buckets(&self) -> String {
        let array_ty = generate_vhdl_array_type_downwards(&self.sliding_window.ty, self.num_buckets - 1);
        format!("signal count_buckets : {};\n\tsignal avg_buckets : {};", array_ty, array_ty)
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
        let avg_default = format!("avg_buckets(avg_buckets'high downto 0) <= (others => {});", ty_default);
        format!("{}\n\t\t\t{}", count_default, avg_default)
    }

    fn create_and_shift_sw_buckets(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        let count = format!("count_buckets <= count_buckets(count_buckets'high-1 downto 0) & {};", ty_default);
        let avg = format!("avg_buckets <= avg_buckets(avg_buckets'high-1 downto 0) & {};", ty_default);
        format!("{}\n\t\t\t\t\t{}", count, avg)
    }

    fn map_and_update_last_sw_bucket(&self) -> String {
        let data_valid = "data_valid_buckets(0) <= '1';".to_string();
        let count = format!(
            "count_buckets(0) <= {}",
            resize_float(
                &self.sliding_window.ty,
                format!("count_buckets(0) + {}", get_count_upd(&self.sliding_window.ty, "count_buckets(0)")),
            )
        );
        let avg = format!(
            "avg_buckets(0) <= {}",
            resize_float(
                &self.sliding_window.ty,
                "(avg_buckets(0) * count_buckets(0) + data_in) / (count_buckets(0) + 1)".to_string()
            )
        );
        format!("{}\n\t\t\t\t{}\n\t\t\t\t{}", data_valid, count, avg)
    }

    fn finalize_sw(&self) -> String {
        let mut fin_avg = "(avg_buckets(0) * count_buckets(0))".to_string();
        for i in 1..self.num_buckets {
            fin_avg = format!("{} + (avg_buckets({}) * count_buckets({}))", fin_avg, i, i);
        }

        let mut fin_count = "count_buckets(0)".to_string();
        for i in 1..self.num_buckets {
            fin_count = format!("{} + count_buckets({})", fin_count, i);
        }

        let div = format!("(({}) / ({}))", fin_avg, fin_count);
        let res = resize_float(&self.sliding_window.ty, div);

        format!("sw_data <= {}", res)
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
    use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowGeneral;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn sliding_window_stream_test() {
        let example_file_content =
            "input a : Float32\noutput b @ 2Hz := a.aggregate(over: 1s, using: avg).defaults(to:0.0)";
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
