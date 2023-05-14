#! /bin/sh

if test $# -eq "2"
then
    if test "$1" = "-i" 
    then
        if test "$2" = "linux" 
        then
            mkdir "./test/benchmarks/linux"
            echo "Downloading Kernel..."
            git clone --depth 1 https://github.com/BurntSushi/linux ./test/benchmarks/linux
        fi
    else
        if test "$2" = "linux" 
        then
            sudo rm -r "./test/benchmarks/linux"
        fi
    fi
fi