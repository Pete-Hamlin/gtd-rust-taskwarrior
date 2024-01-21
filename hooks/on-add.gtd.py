#!/usr/bin/env python

import sys
import json
import subprocess

task = json.loads(sys.stdin.readline())

project = task.get("project")
msg = ""

if 'project' in task:
    msg = subprocess.run(['gtd','add',project], check=True, stdout=subprocess.PIPE, universal_newlines=True)

print(json.dumps(task))
if msg:
    print(msg.stdout)
sys.exit(0)
