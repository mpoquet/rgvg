#! /usr/bin/python3
import re
import sys
import subprocess

tool_list = ["ripgrep", "grep", "ugrep"]
#command_mode = ["release", "debug"]
#exec = ["cg", "vg"]
order_flag = "-o"
documents = "./test/documents/"
results = "./test/results/"
source_comp = "./test/source.txt"

def run(command:str):
    #print("$\x1b[30m", command, "\x1b[39m")
    a = subprocess.run(command, shell=True)
    return a
def q_run(command:str):
    #print("$\x1b[30m", command, "\x1b[39m")
    a = subprocess.run(command, shell=True, capture_output=True)
    return a
def compare(command:str, result_file:str):
    c = run(command + " > " + source_comp)
    c = q_run("diff " + source_comp + " " + results + result_file)
    if c.returncode:
        print("\x1b[31mFailed\x1b[39m")
    else: print("\x1b[32mSucceeded\x1b[39m")
    if len(c.stderr): print(str(c.stderr))
    return c.returncode != 0;

def cg_test_with_tool(tool: str, option_count:str):
    print('\x1b[1mRunning tests for\x1b[34m', tool + '\x1b[39m...\x1b[0m')
    cmd = "./cg/target/release/cg " + order_flag + " --color=no " #We must assume that -o and color function as stated
    cmd += "--tool=" + tool
    d = 0
    for c in option_count:
        print("Running suite", c + "...")
        d += cgsuites[int(c)](cmd)
    return d

basic_pattern = "Sculk Sensor"
basic_dir = documents[:len(documents)-1] #grep doesont like trailing slashes
other_pattern = " l[ea] .*," #note: using l(e|a) makes my grep freak out :(
other_dir = documents + "la-mise-a-jour-sauvage.19.fr"
complex_pattern = "[2-9]0 game ticks"
complex_dir = documents + "mineures/snapshots"

def cg_suite_0(cmd: str):
    #Basic tests
    cmds = []
    cmds.append((cmd + " '" + basic_pattern + "' '" + basic_dir + "' ", "basic_pattern.re"))
    cmds.append((cmd + " '" + other_pattern + "' '" + other_dir + "' ", "other_pattern.re"))
    cmds.append((cmd + " '" + complex_pattern + "' '" + complex_dir + "' ", "advanced_pattern.re"))
    d = 0
    for i in cmds:
        print(">\x1b[30m", i[1], "\x1b[39m", end=" ")
        d += compare(i[0], i[1])
        print(">\x1b[30m", "i_" + i[1], "\x1b[39m", end=" ")
        d += compare(i[0] + "-i", "i_" + i[1])
    return d

superags = [
    ("--include-files='*.??.en'","if"),
    ("--include-dir='snapshots'","id"), #include-dir is fucked up :(
    ("--exclude-files='*.??.en'","ef"),
    ("--exclude-dir='snapshots'","ed"),
]
def cg_suite_1(cmd: str):
    d = 0
    for i in superags:
        cd = cmd + " '" + basic_pattern + "' '" + basic_dir + "' " + i[0]
        fd = i[1] + "_mono_pattern_basic.re"
        print(">\x1b[30m", fd, "\x1b[39m", end=" ")
        d += compare(cd, fd)
    return d

superags2 = [
    ("--include-files='*.20.*'","if"),
    ("--include-dir='candidats'","id"), #include-dir is fucked up :(
    ("--exclude-files='*.20.*'","ef"), #exclude-file with include is borken with classic grep
    ("--exclude-dir='candidats'","ed"),
]
def cg_suite_2(cmd: str):
    d = 0
    for i in superags:
        for j in superags2:
            cd = cmd + " '" + basic_pattern + "' '" + basic_dir + "' " + i[0] + " " + j[0]
            fd = i[1] + j[1] + "_mono_pattern_basic.re"
            print(">\x1b[30m", fd, "\x1b[39m", end=" ")
            d += compare(cd, fd)
    return d

cgsuites = [cg_suite_0, cg_suite_1,  cg_suite_2]


if len(sys.argv) == 1:
    (tool, option_count) = 'all', '012'
if len(sys.argv) == 2: 
    if sys.argv[1] == 'help':
        print('Usage:\x1b[1m', sys.argv[0], '\x1b[0m[tool] [suite*]\n\n')# [exec] [ver.] [v]
        print('  [tool]    Tool to call, "grep", "ripgrep", "ugrep", or "all".\n  default: \x1b[1mall\x1b[0m\n')
        print('  [suite*]  The test suite to run, by id (0,1,2).\n            You can chain multiple suites (012 calls all suites, 000 calls 0 3 times...)\n  default: \x1b[1m012\x1b[0m\n')
        #print('  [exec]    Executable to test, "cg" or "vg".\n  default: \x1b[1mcg\x1b[0m\n')
        #print('  [ver.]    Select build kind, "release" or "debug".\n  default: \x1b[1mrelease\x1b[0m\n')
        #print('  [v]       Verbose.\n  default: \x1b[1mcg\x1b[0m\n')

        exit(0)
    (tool, option_count) = sys.argv[1], '012'
if len(sys.argv) >= 3: 
    (tool, option_count) = sys.argv[1:3]
    #a = []
    #if len(sys.argv) >= 4: 
    #    if sys.argv[3] in command_mode: a.append(sys.argv[3])

if run("cargo build --manifest-path=./cg/Cargo.toml --release -q").returncode:
    print("\x1b[31mBuild failed!\x1b[39m")
else: print("\x1b[32mBuild successfull.\x1b[39m")

if tool == 'all':
    d = 0
    for c in tool_list:
        d += cg_test_with_tool(c,option_count)
    if d > 0: print("\x1b[33mFailed", d, "tests.\x1b[39m")
    else: print("\x1b[32mAll tests successfull.\x1b[39m")
elif tool in tool_list:
    d = cg_test_with_tool(tool,option_count)
    if d > 0: print("\x1b[33mFailed", d, "tests.\x1b[39m")
    else: print("\x1b[32mAll tests successfull.\x1b[39m")
else:
    print("\x1b[31mInvalid tool name:\x1b[39m", tool)



