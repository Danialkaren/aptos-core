#!/bin/sh

# Run a Python binary using Poetry.

bin=$(basename ${0})
exec poetry -C "$(dirname ${0})" run python3 "${bin/.sh/.py}" "${@}"
