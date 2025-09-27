#!/bin/sh

ACTION="serve"

if [ "$1" = "build" ]; then
  ACTION="build"
fi

trunk "$ACTION" --no-default-features --features web
