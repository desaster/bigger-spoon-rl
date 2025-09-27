#!/bin/sh

ACTION="serve"
PARAMS="--no-default-features --features web"

if [ "$1" = "build" ]; then
    ACTION="build"
    PARAMS="$PARAMS --release"
fi

trunk "$ACTION" $PARAMS
