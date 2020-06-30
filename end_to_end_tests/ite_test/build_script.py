import os

#compile vhdl-file
def compile_file(file_name):
    command="ghdl -a --std=08 {}".format(file_name)
    os.system(command)

#elaborate vhdl-file
def elaborate_file(file_name):
    command="ghdl -e --std=08 {}".format(file_name)
    os.system(command)

#run vhdl-file
def run_file(file_name):
    command="ghdl -r --std=08 {} --wave=\"wave_run.ghw\"".format(file_name)
    os.system(command)

#compile phase
#package declaration
compile_file("my_array_package.vhdl")
compile_file("my_math_package.vhdl")
#high-level controller
compile_file("hlc/extInterface.vhdl")
compile_file("hlc/check_new_input.vhdl")
compile_file("hlc/time_unit.vhdl")
compile_file("hlc/scheduler.vhdl")
compile_file("hlc/event_delay.vhdl")
compile_file("hlc/event_scheduler.vhdl")
compile_file("hlc/hl_qinterface.vhdl")
compile_file("hlc/high_level_controller.vhdl")
#low-level controller
compile_file("llc/a_input_stream_entity.vhdl")
compile_file("llc/b_input_stream_entity.vhdl")
compile_file("llc/val_input_stream_entity.vhdl")
compile_file("llc/c_output_stream_entity.vhdl")
compile_file("llc/d_output_stream_entity.vhdl")
compile_file("llc/e_output_stream_entity.vhdl")
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
#test script
compile_file("run_test.vhdl")
#elaborate
elaborate_file("run_test")
#run
run_file("run_test")
