use crate::entity_generator::low_level_controller::sliding_window::sliding_window_avg_entity::SlidingWindowAvgVHDL;
use crate::entity_generator::low_level_controller::sliding_window::sliding_window_avg_float_entity::SlidingWindowAvgFloatVHDL;
use crate::entity_generator::low_level_controller::sliding_window::sliding_window_count_entity::SlidingWindowCountVHDL;
use crate::entity_generator::low_level_controller::sliding_window::sliding_window_integral_entity::SlidingWindowIntegralVHDL;
use crate::entity_generator::low_level_controller::sliding_window::sliding_window_sum_entity::SlidingWindowSumVHDL;
use crate::entity_generator::*;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use crate::Config;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) mod sliding_window_avg_entity;
pub(crate) mod sliding_window_integral_entity;
//pub(crate) mod sliding_window_product_entity;
pub(crate) mod sliding_window_avg_float_entity;
pub(crate) mod sliding_window_count_entity;
pub(crate) mod sliding_window_sum_entity;

pub(crate) trait SlidingWindowTrait {
    fn sw_data_buckets(&self) -> String;
    fn temporary_variables(&self) -> String;
    fn valid_upd(&self) -> String;
    fn set_sw_buckets_to_default(&self) -> String;
    fn create_and_shift_sw_buckets(&self) -> String;
    fn map_and_update_last_sw_bucket(&self) -> String;
    fn finalize_sw(&self) -> String;
    fn finalize_valid(&self) -> String;
}

pub(crate) struct SlidingWindowGeneral<'a, T: SlidingWindowTrait> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) time_per_bucket: u64,
    pub(crate) num_buckets: u32,
    pub(crate) sw_ty: T,
}

impl<'a> SlidingWindowGeneral<'a, SlidingWindowCountVHDL<'a>> {
    pub(crate) fn new_count(
        sliding_window: &'a SlidingWindow,
        ir: &'a RtLolaMir,
        num_buckets: u32,
        time_per_bucket: u64,
    ) -> SlidingWindowGeneral<'a, SlidingWindowCountVHDL<'a>> {
        SlidingWindowGeneral {
            sliding_window,
            ir,
            time_per_bucket,
            num_buckets,
            sw_ty: SlidingWindowCountVHDL::new(sliding_window, num_buckets),
        }
    }
}

impl<'a> SlidingWindowGeneral<'a, SlidingWindowSumVHDL<'a>> {
    pub(crate) fn new_sum(
        sliding_window: &'a SlidingWindow,
        ir: &'a RtLolaMir,
        num_buckets: u32,
        time_per_bucket: u64,
    ) -> SlidingWindowGeneral<'a, SlidingWindowSumVHDL<'a>> {
        SlidingWindowGeneral {
            sliding_window,
            ir,
            time_per_bucket,
            num_buckets,
            sw_ty: SlidingWindowSumVHDL::new(sliding_window, num_buckets),
        }
    }
}

impl<'a> SlidingWindowGeneral<'a, SlidingWindowAvgVHDL<'a>> {
    pub(crate) fn new_avg(
        sliding_window: &'a SlidingWindow,
        ir: &'a RtLolaMir,
        num_buckets: u32,
        time_per_bucket: u64,
    ) -> SlidingWindowGeneral<'a, SlidingWindowAvgVHDL<'a>> {
        SlidingWindowGeneral {
            sliding_window,
            ir,
            time_per_bucket,
            num_buckets,
            sw_ty: SlidingWindowAvgVHDL::new(sliding_window, num_buckets),
        }
    }
}

impl<'a> SlidingWindowGeneral<'a, SlidingWindowAvgFloatVHDL<'a>> {
    pub(crate) fn new_avg_float(
        sliding_window: &'a SlidingWindow,
        ir: &'a RtLolaMir,
        num_buckets: u32,
        time_per_bucket: u64,
    ) -> SlidingWindowGeneral<'a, SlidingWindowAvgFloatVHDL<'a>> {
        SlidingWindowGeneral {
            sliding_window,
            ir,
            time_per_bucket,
            num_buckets,
            sw_ty: SlidingWindowAvgFloatVHDL::new(sliding_window, num_buckets),
        }
    }
}

impl<'a> SlidingWindowGeneral<'a, SlidingWindowIntegralVHDL<'a>> {
    pub(crate) fn new_integral(
        sliding_window: &'a SlidingWindow,
        ir: &'a RtLolaMir,
        num_buckets: u32,
        time_per_bucket: u64,
    ) -> SlidingWindowGeneral<'a, SlidingWindowIntegralVHDL<'a>> {
        SlidingWindowGeneral {
            sliding_window,
            ir,
            time_per_bucket,
            num_buckets,
            sw_ty: SlidingWindowIntegralVHDL::new(sliding_window, num_buckets),
        }
    }
}

impl<T: SlidingWindowTrait> GenerateVhdlCode for SlidingWindowGeneral<'_, T> {
    fn template_name(&self) -> String {
        "sliding_window_general.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        let name = self.ir.get_name_for_stream_ref(self.sliding_window.target);
        let ty = get_str_for_sw_op(self.sliding_window.op);
        format!("{}_{}_{}_sliding_window_entity.vhdl", name, ty, self.sliding_window.reference.idx())
    }
}

impl<T: SlidingWindowTrait> Serialize for SlidingWindowGeneral<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SlidingWindow", 17)?;
        let ty_from_input_stream = self.ir.get_ty_for_stream_ref(self.sliding_window.target);
        // Always the same
        s.serialize_field(
            "name",
            &format!(
                "{}_{}_{}",
                self.ir.get_name_for_stream_ref(self.sliding_window.target),
                get_str_for_sw_op(self.sliding_window.op),
                self.sliding_window.reference.idx()
            ),
        )?;
        let annotation = format!(
            "{}.aggregate(over: {} s, using: {})",
            self.ir.get_name_for_stream_ref(self.sliding_window.target),
            self.sliding_window.duration.as_secs_f64(),
            get_str_for_sw_op(self.sliding_window.op)
        );
        s.serialize_field("annotation", &annotation)?;
        s.serialize_field("input_type_annotation", &ty_from_input_stream.to_string())?;
        s.serialize_field("return_type_annotation", &self.sliding_window.ty.to_string())?;
        s.serialize_field("in_ty", &get_vhdl_type(ty_from_input_stream))?;
        s.serialize_field("sw_ret_ty", &get_vhdl_type(&self.sliding_window.ty))?;
        s.serialize_field("bucket_length", &(&self.num_buckets - 1))?;
        s.serialize_field("sw_zero_assignment", &get_sw_default_value_with_cast(&self.sliding_window.ty, "sw_data"))?;
        s.serialize_field("time_per_bucket", &self.time_per_bucket)?;
        s.serialize_field(
            "time_per_bucket_in_seconds",
            &format!("{}s", self.sliding_window.duration.as_secs_f64() / f64::from(self.num_buckets)),
        )?;
        s.serialize_field("sw_data_buckets", &self.sw_ty.sw_data_buckets())?;
        s.serialize_field("temporary_variables", &self.sw_ty.temporary_variables())?;
        s.serialize_field("valid_upd", &self.sw_ty.valid_upd())?;
        s.serialize_field("set_sw_buckets_to_default", &self.sw_ty.set_sw_buckets_to_default())?;
        s.serialize_field("create_and_shift_sw_buckets", &self.sw_ty.create_and_shift_sw_buckets())?;
        s.serialize_field("map_and_update_last_sw_bucket", &self.sw_ty.map_and_update_last_sw_bucket())?;
        s.serialize_field("finalize_sw", &self.sw_ty.finalize_sw())?;
        s.serialize_field("finalize_valid", &self.sw_ty.finalize_valid())?;
        let streams_where_window_is_used =
            self.ir.get_streams_where_window_is_used(self.ir.sliding_window(self.sliding_window.reference));
        assert_eq!(streams_where_window_is_used.len(), 1, "not implemented, when window is used more than one time");
        let used_stream = self.ir.output(streams_where_window_is_used[0]);
        let mut freq_of_used_stream = None;
        self.ir.time_driven.iter().for_each(|cur| {
            if cur.reference == used_stream.reference {
                freq_of_used_stream = Some(cur);
            }
        });
        let target_annotation =
            format!("{} @{}Hz", used_stream.name, freq_of_used_stream.expect("should not happen").frequency.value);
        s.serialize_field("source_stream", &target_annotation)?;
        s.end()
    }
}

pub(crate) fn generate_sliding_window(sw: &SlidingWindow, config: &Config) {
    let mut target = config.target.clone();
    target.push("llc/");
    let tera_files = config.templates.clone() + "/low_level_controller/sliding_windows/*";
    let tera = tera::compile_templates!(&tera_files);

    let num_buckets = config.ir.get_num_buckets(sw);
    let time_per_bucket = (sw.duration.as_nanos() / u128::from(num_buckets)) as u64;

    match sw.op {
        WindowOperation::Count => VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_count(sw, &config.ir, num_buckets, time_per_bucket),
            &tera,
            &target,
        ),
        WindowOperation::Sum => VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_sum(sw, &config.ir, num_buckets, time_per_bucket),
            &tera,
            &target,
        ),
        WindowOperation::Average => {
            if is_float_type(&sw.ty).is_some() {
                VHDLGenerator::generate_and_create(
                    &SlidingWindowGeneral::new_avg_float(sw, &config.ir, num_buckets, time_per_bucket),
                    &tera,
                    &target,
                )
            } else {
                VHDLGenerator::generate_and_create(
                    &SlidingWindowGeneral::new_avg(sw, &config.ir, num_buckets, time_per_bucket),
                    &tera,
                    &target,
                )
            }
        }
        WindowOperation::Integral => VHDLGenerator::generate_and_create(
            &SlidingWindowGeneral::new_integral(sw, &config.ir, num_buckets, time_per_bucket),
            &tera,
            &target,
        ),
        _ => unimplemented!(""),
    };
}
