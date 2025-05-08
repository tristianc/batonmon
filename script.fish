#!/usr/bin/fish

set -x display 2
set -x current (ddcutil -d 2 getvcp 60 | sed -n "s/.*(sl=\(.*\))/\1/p")
if test $current = "0x19"
    set -gx output "0x0f"
else if test $current = "0x0f" 
    set -gx output "0x19"
else
    echo "Unknown current output: $current"
    exit -1
end
echo "output is $output"
ddcutil -d $display setvcp 60 $output
