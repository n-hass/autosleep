#!/bin/bash

if [[ $(pgrep -f code-server) ]]; then
	exit 0
else
	exit 1
fi
