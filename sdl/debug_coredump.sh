#!/bin/bash

# Fetches and starts core dump debugging via gdb

cp /tmp/core .
gdb target/debug/songbird_sdl core
