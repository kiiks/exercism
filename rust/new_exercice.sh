#!/bin/bash
# Need to be ran with `source` command to actually change current directory. (ex: `source new_exercice.sh`)

if [ $# -ne 3 ]; then
    echo "Usage: $0 <path> <language_name> <exercise_name>"
    exit 1
fi

path="$1"
language="$2"
exercise="$3"

cd $path || exit 1

exercism download --track=rust --exercise="$exercise" || exit 1

cd $exercise || exit 1

echo "Exercice '$exercise' downloaded successfully and working directory changed."