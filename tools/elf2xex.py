import re
import subprocess
import sys


def parse_hex(data):
    return bytes(int(data[i:i+2], 16) for i in range(0, len(data))[::2])

input_file = sys.argv[1]

SECTIONS = [".xex_header", ".text", ".rodata", ".debug_gdb_scripts", ".data"]

sections_str = ' '.join(f"--section={section}" for section in SECTIONS)

CMD = f"llvm-objdump --full-contents {sections_str} {input_file}"

DATA_RE = re.compile("^ [0-9a-f]{4} (([0-9a-f]{1,8} )+)")

output = subprocess.getoutput(CMD)
out_data = []

for line in output.split("\n"):
    match = DATA_RE.match(line)
    if match:
        data = match.group(1).replace(" ", "")
        out_data.append(parse_hex(data))

if not out_data:
    print(output)
    sys.exit(1)

with open(f"{input_file}.xex", "wb") as f:
    for data in out_data:
        f.write(data)
