set print asm-demangle on
set pagination off
target extended-remote :2331
monitor semihosting enable
monitor semihosting IOClient 3
monitor reset
load
continue
# break main
# continue
