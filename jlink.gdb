# Run the jlink debugger in a separate terminal: `JLinkGDBServer -device da14531 -if SWD`
target extended-remote :2331

set print asm-demangle on

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
#break rust_begin_unwind

load

# Start process but immediately halt the processor
stepi
