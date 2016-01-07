#!/bin/sh

# `jar cfm kwho.jar kwho.manifest kwho.class kwho.java`

# Requires the jar file to be in the same directory as this file.
dir=`dirname "$0"`
file=`basename "$0"`

# Should chase symlinks.  Sigh.
exec java -jar "$dir"/kwho.jar

# cd "$dir"
# perl -w -e 'print readlink('\""$file"\"'), "\n"'
# 
# exit
