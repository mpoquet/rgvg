#! /usr/bin/python3
import re
import sys
import subprocess

tool_list = ["ripgrep", "grep", "ugrep"]
#command_mode = ["release", "debug"]
#exec = ["cg", "vg"]

def run(command:str):
    subprocess.run(command, shell=True)

def test_with_tool(tool: str, option_count:str):
    print('\x1b[1mRunning tests for\x1b[34m', tool + '\x1b[39m...\x1b[0m')
    cmd = "./cg/target/release/cg "
    

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

if tool == 'all':
    for c in tool_list:
        test_with_tool(c,option_count)
elif tool in tool_list:
    test_with_tool(tool,option_count)



