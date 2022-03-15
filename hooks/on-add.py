#!/usr/bin/env python

import sys
import json
import subprocess

task = json.loads(sys.stdin.readline())
print(json.dumps(task))

project = task.get("project")
if project:
    subprocess.run(['gtd','add',project], check=True, stdout=subprocess.PIPE, universal_newlines=True)

sys.exit(0)