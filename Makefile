.PHONY: start

start:
	qemu-system-x86_64 -drive format=raw,\
	file=target/x86_64-fly_os/debug/bootimage-fly_os.bin

