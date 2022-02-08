#!/bin/bash
KERNEL_DISK=build/kernel.img
META=meta/x86_64

# Update the disk
$META/clean.sh > /dev/null
$META/build_limine.sh > /dev/null

# Run QEMU
qemu-system-x86_64 -m 4G -serial stdio -no-reboot -enable-kvm -d cpu_reset -hda $KERNEL_DISK