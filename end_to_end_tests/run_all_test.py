#!/usr/bin/env python3
import subprocess
from pathlib import Path
import sys
import argparse
import platform
import os
import re

EXIT_FAILURE = 1

tests = ["bool_and_comp_test", "different_frequencies", "ite_test", "loop_test", "math_operators", "sliding_window_test", "float_math_operators"]

def print_fail(message, end='\n'):
    sys.stdout.write('\x1b[1;31m' + message.rstrip() + '\x1b[0m' + end)

def print_bold(message, end='\n'):
    sys.stdout.write('\x1b[1;37m' + message.rstrip() + '\x1b[0m' + end)
        
def print_pass(message, end='\n'):
    sys.stdout.write('\x1b[1;32m' + message.rstrip() + '\x1b[0m' + end)

parser = argparse.ArgumentParser(description='Run end-end tests for FPGA-Lola')


running_on_windows = platform.system() == "Windows"
executable_name = "rtlola-compiler-vhdl.exe" if running_on_windows else "rtlola-compiler-vhdl"

build_mode = os.getenv("BUILD_MODE", default="debug")

repo_base_dir = Path(".").resolve()

executable_path = repo_base_dir / "target" / build_mode / executable_name

executable_path_string = str(executable_path)

if build_mode == "debug":
    cargo_build = subprocess.run(["cargo", "build"], cwd=str(repo_base_dir))
elif build_mode == "release":
    cargo_build = subprocess.run(["cargo", "build", "--release"], cwd=str(repo_base_dir))
else:
    print_fail("invalid BUILD_MODE '{}'".format(build_mode))
    sys.exit(EXIT_FAILURE)
if cargo_build.returncode != 0:
    print_fail("Build failed")
    sys.exit(EXIT_FAILURE)

total_number_of_tests = 0
failed_tests = 0
crashed_tests = 0
passed_tests = 0

test_dir = repo_base_dir / "end_to_end_tests"
template_dir = repo_base_dir / "templates"
test_spec_dir = test_dir / "specs"

tests_passed = []
tests_crashed = []
tests_failed = []
return_code = 0

for test in tests:
    total_number_of_tests += 1
    print("========================================================================")
    print_bold("{}:".format(test))
    spec_file = test_spec_dir / "{}.lola".format(test)
    target_dir = test_dir / test
    some_thing_wrong = False
    compile_file_result = None
    try:
        compile_file_result = subprocess.run([executable_path_string, spec_file, target_dir, template_dir,"--offline"],cwd=str(repo_base_dir), timeout=10)
    except:
        tests_crashed.append(test)
        print_fail("Could no compile file")
        some_thing_wrong = True
    if compile_file_result is not None:
        if compile_file_result.returncode == 0:
            run_test_result = None
            try:
                run_test_result = subprocess.run(["python3", "build_script.py"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=str(target_dir), universal_newlines=True, timeout=10)
            except subprocess.TimeoutExpired:
                tests_crashed.append(test)
                print_fail("Could not run compiled file")
                some_thing_wrong = True
            if run_test_result is not None:
                if run_test_result.returncode == 0:
                    lines = run_test_result.stdout.split("\n")
                    if len(lines) != 1:
                        some_thing_wrong = True
                        tests_failed.append(test)
                        for line in lines:
                            if line == "":
                                continue
                            print_fail(line)
            if some_thing_wrong:
                print_fail("FAIL")
                return_code = 1
            else:
                tests_passed.append(test)
                print_pass("PASS")

            print("")
        else:
            print_fail("FAIL")
            tests_crashed.append(test)

print("========================================================================")
print("Total tests: {}".format(total_number_of_tests))
print_pass("Tests passed: {}".format(len(tests_passed)))
if len(tests_crashed) > 0:
    print_fail("Tests crashed: {}".format(len(tests_crashed)))
    for test in tests_crashed:
        print_fail("\t{}".format(test))
if len(tests_failed) > 0:
    print_fail("Tests failed: {}".format(len(tests_failed)))
    for test in tests_failed:
        print_fail("\t{}".format(test))
print("========================================================================")
sys.exit(return_code)            
