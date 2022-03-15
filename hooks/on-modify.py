#!/usr/bin/env python

import sys
import json
import subprocess

task = json.loads(sys.stdin.readline())

project = task.get("project")
if project:
    subprocess.run(['gtd','insert',project], check=True, stdout=subprocess.PIPE, universal_newlines=True)