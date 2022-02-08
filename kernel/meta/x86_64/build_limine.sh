#!/bin/bash
mkdir build

DISK=build/kernel.img

# Variables used under
KERNEL_ELF=target/x86_64-adamant/debug/kernel
ECHFS_UTILS=thirdparty/echfs/echfs-utils
ECHFS_ROOT=thirdparty/echfs
LIMINE_PATH=thirdparty/limine
LIMINE_CFG=meta/x86_64/limine.cfg

ROOT=./build/temp
mkdir -p $ROOT

cp $KERNEL_ELF $LIMINE_CFG $LIMINE_PATH/limine.sys $LIMINE_PATH/limine-cd.bin $LIMINE_PATH/limine-eltorito-efi.bin $ROOT/

xorriso -as mkisofs -b limine-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-eltorito-efi.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    $ROOT/ -o $DISK

rm -rf $ROOT

# Build image



$LIMINE_PATH/limine-install-linux-x86_64 $DISK