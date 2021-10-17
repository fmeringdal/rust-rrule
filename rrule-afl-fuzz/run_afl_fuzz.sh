#!/bin/sh

# Stop on error
set -e

# Install https://crates.io/crates/afl
# Need to be build with same rust version as it is running
cargo install afl || true

# Create folder
FOLDER_IN="in_raw_rrule"
FOLDER_OUT="out"

# Build
cargo afl build --features "no-validation-limits"
# Fuzz binary
BIN_PATH="../target/debug/rrule-afl-fuzz"
# Env variable to set for all instances
FUZZ_ENV_VAR="AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1 AFL_SKIP_CPUFREQ=1 AFL_MAP_SIZE=131072"
# What command should be used to open new terminal windows
#
# For Kde -> `konsole`
# For genome -> `genome-terminal`
# For xfce4 -> `xfce4-terminal`
# For chinnamon -> `x-terminal-emulator`
# For mate -> `mate-terminal --window`
# For unity -> `gnome-terminal --profile=Default`
# For pathenom -> `pantheon-terminal -w ''`
# Source: https://superuser.com/a/1672713/516502
USE_TERMINAL_COMMAND="x-terminal-emulator -e"
# Amount of CPU cores that you want to use to fuzz
# This will max out all cores you give it, so leave at least 1 core over so system is still usable.
USE_CPU_CORES=1

# Run fuzzer
if [ "$USE_CPU_CORES" = "1" ]; then
    # Start just 1 task and just run in current terminal
    eval "$FUZZ_ENV_VAR cargo afl fuzz -i $FOLDER_IN -o $FOLDER_OUT $BIN_PATH"
else
    # Start multiple terminals to run commands.
    # First will be the primary instance others are secondary (master-slave)
    eval "$USE_TERMINAL_COMMAND \"$FUZZ_ENV_VAR cargo afl fuzz -i $FOLDER_IN -o $FOLDER_OUT -M fuzzer1 $BIN_PATH\""
    # Wait for main service to start
    sleep 5
    for i in $(seq 2 $USE_CPU_CORES)
    do
        echo "$USE_TERMINAL_COMMAND \"$FUZZ_ENV_VAR cargo afl fuzz -i $FOLDER_IN -o $FOLDER_OUT -S fuzzer$i $BIN_PATH\""
        eval "$USE_TERMINAL_COMMAND \"$FUZZ_ENV_VAR cargo afl fuzz -i $FOLDER_IN -o $FOLDER_OUT -S fuzzer$i $BIN_PATH\""
    done
fi
