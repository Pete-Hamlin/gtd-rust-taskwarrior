#!/usr/bin/env python

import sys
import json
import subprocess

_ = json.loads(sys.stdin.readline())
new_task = json.loads(sys.stdin.readline())
print(json.dumps(new_task))

project = new_task.get("project")
if project:
    subprocess.run(['gtd','add',project], check=True, stdout=subprocess.PIPE, universal_newlines=True)

sys.exit(0)