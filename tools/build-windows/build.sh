#!/usr/bin/env bash
VER=`git describe --tags --exact-match 2> /dev/null || git symbolic-ref -q --short HEAD || git rev-parse --short HEAD`
ARCH=`uname -m`
KERNEL="windows"
TARGET="$ARCH-$KERNEL"

function cg {
  function git_root {
    local top; top="$(git rev-parse --show-cdup)"
    top="${top:-./}"
    local super_root; super_root="$(git rev-parse --show-superproject-working-tree)"
    if [[ "$super_root" ]]; then
      printf '%s' "$top../"
      ( cd "$top../" && git_root || return )
    fi
    printf '%s' "$top"
  }
  local tree_root
  tree_root="$(git_root)"
  [[ "x${tree_root}" != "x./" ]] && cd "${tree_root}" && return || return 0
}
cg

cargo build --release --locked

rm -rf ./tmp/
mkdir -p tmp/
cp target/release/wolkenwelten.exe tmp/wolkenwelten.exe && \
rm -rf ./dist/ && \
mkdir dist/ && \
cd "tmp/" && 7z a "../dist/wolkenwelten-$VER-$TARGET.zip" ./wolkenwelten.exe
