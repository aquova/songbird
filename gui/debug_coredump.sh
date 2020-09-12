#!/bin/bash

# Fetches and starts coredump debugging via gdb
rm -rf core
mv /tmp/core .
gdb target/debug/songbird_gui core
