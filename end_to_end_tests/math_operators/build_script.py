import os
#variable for ghdl alias; TODO: write a function that finds this alias;
ghdl="/Users/janbaumeister/Documents/Apps_Preferences/GHDL/ghdl-0.35-mcode-macosx/bin/ghdl"

#compile vhdl-file
def compile_file(file_name):
    format_list=[ghdl, file_name]
    command="{} -a --std=08 {}".format(*format_list)
    os.system(command)

#elaborate vhdl-file
def elaborate_file(file_name):
    format_list=[ghdl, file_name]
    command="{} -e --std=08 {}".format(*format_list)
    os.system(command)

#run vhdl-file
def run_file(file_name):
    format_list=[ghdl, file_name]
    command="{} -r --std=08 {} --wave=\"wave_run.ghw\"".format(*format_list)
    os.system(command)

#compile phase
#package_declaration
compile_file("my_array_package.vhdl")
compile_file("my_math_package.vhdl")
#timing manager
compile_file("hlc/extInterface.vhdl")
compile_file("hlc/check_new_input.vhdl")
compile_file("hlc/time_unit.vhdl")
compile_file("hlc/scheduler.vhdl")
compile_file("hlc/event_delay.vhdl")
compile_file("hlc/event_scheduler.vhdl")
compile_file("hlc/hl_qinterface.vhdl")
compile_file("hlc/high_level_controller.vhdl")
#low_level_controller

compile_file("llc/a_input_stream_entity.vhdl")
compile_file("llc/b_input_stream_entity.vhdl")
compile_file("llc/plus_op_output_stream_entity.vhdl")
compile_file("llc/minus_op_output_stream_entity.vhdl")
compile_file("llc/mult_op_output_stream_entity.vhdl")
compile_file("llc/div_op_output_stream_entity.vhdl")
compile_file("llc/mod_op_output_stream_entity.vhdl")
#compile_file("Evaluator/pow_op_output_stream_entity.vhdl")
compile_file("llc/func_abs_output_stream_entity.vhdl")
compile_file("llc/func_sqrt_output_stream_entity.vhdl")
compile_file("llc/counter_output_stream_entity.vhdl")
compile_file("llc/evaluator.vhdl")
compile_file("llc/low_level_controller.vhdl")
#queue
compile_file("queue/queue.vhdl")
#monitor
compile_file("monitor.vhdl")
#implementation
compile_file("pre_processing/clock_pre_processing.vhdl")
compile_file("pre_processing/input_pre_processing.vhdl")
compile_file("implementation.vhdl")
#test_script
compile_file("run_test.vhdl")
#elaborate
elaborate_file("run_test")
#run
run_file("run_test")