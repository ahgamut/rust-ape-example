#!/bin/bash
set -eu

COSMO="${COSMO:-/opt/cosmo}"
ARCH="${ARCH:-x86_64}"

declare -a args
args=()
for o in "$@" ; do
    case $o in
        "-lunwind") continue;;
        "-Wl,-Bdynamic") continue;;
        "-Wl,-Bstatic") continue;;
    esac
    args+=("$o")
done

$COSMO/bin/$ARCH-unknown-cosmo-cc "${args[@]}"
