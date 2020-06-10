#!/bin/bash

# Fetches and starts core dump debugging via gdb

rm -f core
mv /tmp/core .
gdb target/debug/songbird_sdl core
