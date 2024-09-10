bitbox02 bluetooth firmware

# Connect to device

Run Jlink gdb server

```
JLinkGDBServer -device da14531 -if SWD -ir
```

Run firmware

```
make run
```
