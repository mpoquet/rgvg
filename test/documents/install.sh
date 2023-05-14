#! /bin/sh

if test $# -eq "2"
then
    if test "$1" = "-i" 
    then
        if test "$2" = "linux" 
        then
            mkdir "./test/documents/linux"
            echo "Downloading Kernel..."
            git clone --depth 1 https://github.com/BurntSushi/linux ./test/documents/linux
        fi
    else
        if test "$2" = "linux" 
        then
            sudo rm -r "./test/documents/linux"
        fi
    fi
fi