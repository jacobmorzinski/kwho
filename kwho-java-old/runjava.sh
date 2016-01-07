#!/bin/bash
#
# runjava.sh: executes the JAR file whose name is used to run this script
#

BASE="$(basename $0)"
DIR="$(dirname $0)"
JARFILE="$DIR/jars/$BASE.jar"

[ -f "$JARFILE" ] && exec java -jar "$JARFILE" "$@"
echo runjava.sh: $BASE: command not found
exit 127
