#!/usr/bin/env python

import os, re

klist = "/usr/athena/bin/klist"
pattern = re.compile(r"^(?:default )?principal: (.*)@.*$", re.I)

handle=os.popen(klist)
for line in handle:
    if (pattern.search(line)):
        line = re.sub(pattern, r"\1", line)
        print line,
        break
