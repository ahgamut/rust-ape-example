#!/usr/bin/env bash
set -eux

gcc -g -Os -static -nostdlib -nostdinc \
  -fno-pie -no-pie -mno-red-zone \
  -fno-omit-frame-pointer -pg -mnop-mcount \
  -o ./stubs.o -c ./stubs.c
