#!/usr/bin/env bash
if [ "${PWD##*/}" == "intro_to_algo" ]; then
	./build/insertion_sort
else
	echo "You are not in the root project directory. Current directory is: $CURRENT_DIR"
fi
