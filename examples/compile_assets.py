#!/bin/python

import os
from pathlib import Path
import subprocess
import shutil

script_dir = os.path.dirname(__file__)
gfx_dir = os.path.join(script_dir, "gfx")
output_path = os.path.join(script_dir, "romfs", "gfx")

# Remove anything in the output dir
shutil.rmtree(output_path)
os.makedirs(output_path, exist_ok=True)

files = Path(gfx_dir).glob("*.t3s")
for file in files:
    path = str(file)

    file_name = Path(path).stem + ".t3x"

    subprocess.run([
        "tex3ds",
        "-i",
        path,
        "-o",
        os.path.join(output_path, file_name)
    ], cwd=gfx_dir)
