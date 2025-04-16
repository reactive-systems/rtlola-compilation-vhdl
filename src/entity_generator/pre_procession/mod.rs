use crate::entity_generator::VHDLGenerator;
use crate::static_constants::NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_OFFLINE;
use crate::static_constants::NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_ONLINE;
use crate::Config;

pub(crate) mod clock_pre_processing_offline;
pub(crate) mod clock_pre_processing_online;
pub(crate) mod input_pre_processing;

pub(crate) fn generate_preprocessing(config: &Config) {
    let mut target = config.target.clone();
    target.push("pre_processing/");
    let tera_files = config.templates.clone() + "/pre_processing/*";
    let tera = tera::compile_templates!(&tera_files);
    if config.mode {
        VHDLGenerator::generate_and_create(
            &clock_pre_processing_online::ClockPreProcessingOnline::new(NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_ONLINE),
            &tera,
            &target,
        );
    } else {
        VHDLGenerator::generate_and_create(
            &clock_pre_processing_offline::ClockPreProcessingOffline::new(NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_OFFLINE),
            &tera,
            &target,
        );
    }
    VHDLGenerator::generate_and_create(&input_pre_processing::InputPreProcessing::new(&config.ir), &tera, &target);
}
