# To replace docker with podman set `DOCKER=podman` in your environment
DOCKER ?= docker

.PHONY: dockerdev
dockerdev:
	@./scripts/dockerenv.sh

.PHONY: dockerinit
dockerinit:
	${DOCKER} build -t shiftcrypto/da14531 .

.PHONY: dockerstop
dockerstop:
	@./scripts/dockerenv.sh stop

.PHONY: gdb-server
gdb-server:
	JLinkGDBServer -device da14531 -if SWD -ir

.PHONY: rtt-client
rtt-client:
	telnet localhost 19021
