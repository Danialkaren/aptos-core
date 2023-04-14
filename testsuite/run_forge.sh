#!/bin/bash

# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

# A light wrapper for the new forge python script

echo "Warning: run_forge.sh is deprecated. Please use forge.sh instead."
echo "Executing testsuite/forge.sh test $@"
exec testsuite/forge.sh test "$@"
