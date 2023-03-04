set print asm-demangle on
target extended-remote 172.23.224.1:2331
monitor semihosting enable
monitor semihosting IOClient 3
monitor reset
load
break main
continue
