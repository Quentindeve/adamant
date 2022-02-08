#!/bin/bash
mkdir build

DISK=build/kernel.img
# Creates the image file.
dd if=/dev/zero bs=8M count=0 seek=64 of=$DISK
parted -s $DISK mklabel msdos
parted -s $DISK mkpart primary 1 100%
parted -s $DISK set 1 boot on

# Variables used under
KERNEL_ELF=target/x86_64-adamant/debug/kernel
ECHFS_UTILS=thirdparty/echfs/echfs-utils
ECHFS_ROOT=thirdparty/echfs
LIMINE_PATH=thirdparty/limine
LIMINE_CFG=meta/x86_64/limine.cfg

# Build echfs-utils
cd $ECHFS_ROOT
make utils
cd ../../

# Use echfs-utils to import everything on image
$ECHFS_UTILS -m -p0 $DISK quick-format 32768
$ECHFS_UTILS -m -p0 $DISK import $LIMINE_PATH/limine.sys limine.sys
$ECHFS_UTILS -m -p0 $DISK import $LIMINE_CFG limine.cfg
$ECHFS_UTILS -m -p0 $DISK import $KERNEL_ELF kernel.elf

$LIMINE_PATH/limine-install-linux-x86_64 $DISK