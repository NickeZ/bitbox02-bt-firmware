# bitbox02 bluetooth firmware

## Dependencies

* ARM GCC Toolchain
* da14531 SDK

## Connect to device

Run Jlink gdb server

```
cd bitbox02-bt
make gdb-server
```

Run gdb, which will load firmware into RAM and start execution.

```
cd bitbox02-bt
make run
```
