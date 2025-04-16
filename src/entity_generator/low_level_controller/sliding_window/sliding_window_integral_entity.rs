use crate::entity_generator::low_level_controller::sliding_window::SlidingWindowTrait;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;

pub(crate) struct SlidingWindowIntegralVHDL<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) num_buckets: u32,
}

impl<'a> SlidingWindowIntegralVHDL<'a> {
    pub(crate) fn new(sliding_window: &'a SlidingWindow, num_buckets: u32) -> SlidingWindowIntegralVHDL<'a> {
        SlidingWindowIntegralVHDL { sliding_window, num_buckets }
    }

    fn get_ty_as_string(ty: &Type) -> String {
        match ty {
            Type::Int(_) => "signed".to_string(),
            Type::UInt(_) => "unsigned".to_string(),
            Type::Option(op_ty) => SlidingWindowIntegralVHDL::get_ty_as_string(op_ty),
            Type::Float(_) => "signed".to_string(),
            _ => unimplemented!(),
        }
    }

    fn get_product_ty(ty: &Type) -> String {
        let ty_as_string = SlidingWindowIntegralVHDL::get_ty_as_string(ty);
        let size = get_value_for_Ty(ty) * 2 + 1;
        format!("{}({} downto 0)", ty_as_string, size)
    }
}

impl SlidingWindowTrait for SlidingWindowIntegralVHDL<'_> {
    fn sw_data_buckets(&self) -> String {
        let array_ty_data = generate_vhdl_array_type_downwards(&self.sliding_window.ty, self.num_buckets - 1);
        let array_ty_time = generate_vhdl_array_type_downwards(&Type::UInt(UIntTy::UInt64), self.num_buckets - 1);
        let lhs =
            format!("signal lhs_value_buckets : {};\n\tsignal lhs_time_buckets : {};", array_ty_data, array_ty_time);
        let rhs =
            format!("signal rhs_value_buckets : {};\n\tsignal rhs_time_buckets : {};", array_ty_data, array_ty_time);
        let volume = format!("signal volume_buckets : {};", array_ty_data);
        format!("{}\n\t{}\n\t{}", lhs, rhs, volume)
    }

    fn temporary_variables(&self) -> String {
        let var_ty = get_vhdl_type(&self.sliding_window.ty);
        let volume_between_two_samples_ty = SlidingWindowIntegralVHDL::get_product_ty(&self.sliding_window.ty);
        let half_time_diff = "variable half_time_diff: std_logic_vector(63 downto 0);".to_string();
        let volume_between_two_samples =
            format!("variable volume_between_two_samples : {};", volume_between_two_samples_ty);
        let data_valid_temp = "variable data_valid_temp : std_logic;".to_string();
        let lhs_value_temp = format!("variable lhs_value_temp : {};", var_ty);
        let lhs_time_temp = "variable lhs_time_temp : unsigned(63 downto 0);".to_string();
        //        let rhs_value_temp = format!("variable rhs_value_temp : {};", var_ty);
        //        let rhs_time_temp = "variable rhs_time_temp : unsigned(63 downto 0);".to_string();
        let volume_temp = format!("variable volume_temp : {};", var_ty);
        //        format!("{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}", half_time_diff, volume_between_two_samples, data_valid_temp, lhs_value_temp, lhs_time_temp, rhs_value_temp, rhs_time_temp, volume_temp)
        format!(
            "\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t",
            half_time_diff, volume_between_two_samples, data_valid_temp, lhs_value_temp, lhs_time_temp, volume_temp
        )
    }

    fn valid_upd(&self) -> String {
        "'0'".to_string()
    }

    fn set_sw_buckets_to_default(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        let time_default = get_sw_default_value_with_cast(&Type::UInt(UIntTy::UInt64), "time_in");
        let lhs_value_default =
            format!("lhs_value_buckets(lhs_value_buckets'high downto 0) <= (others => {});", ty_default);
        let lhs_time_default =
            format!("lhs_time_buckets(lhs_time_buckets'high downto 0) <= (others => {});", time_default);
        let rhs_value_default =
            format!("rhs_value_buckets(rhs_value_buckets'high downto 0) <= (others => {});", ty_default);
        let rhs_time_default =
            format!("rhs_time_buckets(rhs_time_buckets'high downto 0) <= (others => {});", time_default);
        let volume_default = format!("volume_buckets(volume_buckets'high downto 0) <= (others => {});", ty_default);
        format!(
            "{}\n\t\t\t{}\n\t\t\t{}\n\t\t\t{}\n\t\t\t{}",
            lhs_value_default, lhs_time_default, rhs_value_default, rhs_time_default, volume_default
        )
    }

    fn create_and_shift_sw_buckets(&self) -> String {
        let ty_default = get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data");
        let lhs_value_shift =
            format!("lhs_value_buckets <= lhs_value_buckets(lhs_value_buckets'high-1 downto 0) & {};", ty_default);
        let lhs_time_shift =
            "lhs_time_buckets <= lhs_time_buckets(lhs_time_buckets'high-1 downto 0) & time_in;".to_string();
        let rhs_value_shift =
            format!("rhs_value_buckets <= rhs_value_buckets(rhs_value_buckets'high-1 downto 0) & {};", ty_default);
        let rhs_time_shift =
            "rhs_time_buckets <= rhs_time_buckets(rhs_time_buckets'high-1 downto 0) & time_in;".to_string();
        let volume_shift =
            format!("volume_buckets <= volume_buckets(volume_buckets'high-1 downto 0) & {};", ty_default);
        format!(
            "{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}",
            lhs_value_shift, lhs_time_shift, rhs_value_shift, rhs_time_shift, volume_shift
        )
    }

    fn map_and_update_last_sw_bucket(&self) -> String {
        let bit_range = get_value_for_Ty(&self.sliding_window.ty);

        let data_valid_cons = "data_valid_buckets(0) <= '1';".to_string();
        let lhs_value_cons = "lhs_value_buckets(0) <= data_in;".to_string();
        let lhs_time_cons = "lhs_time_buckets(0) <= time_in;".to_string();
        let rhs_value_cons = "rhs_value_buckets(0) <= data_in;".to_string();
        let rhs_time_cons = "rhs_time_buckets(0) <= time_in;".to_string();
        let cons = format!(
            "{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}",
            data_valid_cons, lhs_value_cons, lhs_time_cons, rhs_value_cons, rhs_time_cons
        );

        let half_time_diff_alt = "half_time_diff := std_logic_vector((time_in - rhs_time_buckets(0)) / 2);".to_string();
        let volume_between_two_samples_ty_alt = format!(
            "volume_between_two_samples := (rhs_value_buckets(0) + data_in) * {}(half_time_diff({} downto 0));",
            SlidingWindowIntegralVHDL::get_ty_as_string(&self.sliding_window.ty),
            bit_range
        );
        let rhs_value_alt = "rhs_value_buckets(0) <= data_in;".to_string();
        let rhs_time_alt = "rhs_time_buckets(0) <= time_in;".to_string();
        let volume_alt =
            format!("volume_buckets(0) <= volume_buckets(0) + volume_between_two_samples({} downto 0);", bit_range);
        let alt = format!(
            "{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}",
            half_time_diff_alt, volume_between_two_samples_ty_alt, rhs_value_alt, rhs_time_alt, volume_alt
        );
        format!(
            "if (data_valid_buckets(0) = '0') then\n\t\t\t\t\t{}\n\t\t\t\telse\n\t\t\t\t\t{}\n\t\t\t\tend if;",
            cons, alt
        )
    }

    fn finalize_sw(&self) -> String {
        let bit_range = get_value_for_Ty(&self.sliding_window.ty);

        let comment_init = "-- Iterate Over All Buckets, Beginning with the Last Timestamp".to_string();
        let data_valid_temp_init = "data_valid_temp := data_valid_buckets(0);".to_string();
        let lhs_value_temp_init = "lhs_value_temp := lhs_value_buckets(0);".to_string();
        let lhs_time_temp_init = "lhs_time_temp := lhs_time_buckets(0);".to_string();
        let volume_temp_init = "volume_temp := volume_buckets(0);".to_string();
        let init = format!(
            "{}\n\t\t\t\t{}\n\t\t\t\t{}\n\t\t\t\t{}\n\t\t\t\t{}",
            comment_init, data_valid_temp_init, lhs_value_temp_init, lhs_time_temp_init, volume_temp_init
        );

        let lhs_value_temp = "lhs_value_temp := lhs_value_buckets(i);".to_string();
        let lhs_time_temp = "lhs_time_temp := lhs_time_buckets(i);".to_string();

        let comment_cons = "-- Reduction with Bot for the Pre-aggregated Values".to_string();
        let data_valid_cons = "data_valid_temp := data_valid_buckets(i);".to_string();
        let volume_temp_cons = "volume_temp := volume_buckets(i);".to_string();
        let cons = format!(
            "{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}",
            comment_cons, data_valid_cons, lhs_value_temp, lhs_time_temp, volume_temp_cons
        );

        let comment_alt = "-- Reduction with Valid Values on Both Sides".to_string();
        let half_time_diff_alt =
            "half_time_diff := std_logic_vector((lhs_time_temp - rhs_time_buckets(i)) / 2);".to_string();
        let volume_between_two_samples_ty_alt = format!(
            "volume_between_two_samples := (lhs_value_temp + data_in) * {}(half_time_diff({} downto 0));",
            SlidingWindowIntegralVHDL::get_ty_as_string(&self.sliding_window.ty),
            bit_range
        );
        let volume_alt = format!("volume_temp := volume_temp + volume_between_two_samples({} downto 0);", bit_range);
        let alt = format!(
            "{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}\n\t\t\t\t\t{}",
            comment_alt,
            half_time_diff_alt,
            volume_between_two_samples_ty_alt,
            volume_alt,
            lhs_value_temp,
            lhs_time_temp
        );

        let computation = format!("{}\n\t\t\t\tfor i in 1 to {} loop\n\t\t\t\t\tif data_valid_temp = '0' then \n\t\t\t\t\t\t{}\n\t\t\t\t\telsif data_valid_buckets(i) = '0' then\n\t\t\t\t\t\t{}\n\t\t\t\t\tend if;\n\t\t\t\tend loop;", init, self.num_buckets-1, cons,alt);
        format!("{}\n\t\t\t\tsw_data <= volume_temp;", computation)
    }

    fn finalize_valid(&self) -> String {
        "data_valid_temp".to_string()
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
        let example_file_content = "input a : Int8\noutput b @ 2Hz := a.aggregate(over: 1s, using: integral)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let tera: Tera = compile_templates!("templates/low_level_controller/sliding_windows/*");
        VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_integral(sw, &lola_instance, 4, 4),
            &tera,
            &PathBuf::from("target/test_files"),
        );
    }
}
