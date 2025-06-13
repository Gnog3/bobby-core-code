import os, sys, base64
from pathlib import Path
if len(sys.argv) < 2:
    print('error: no arguments')
    exit()

def exit_if_error(r):
    if r != 0:
        print(f'Exit with code {r}')
        exit(r)

compiler_bin = Path('compiler/bin')

gcc = compiler_bin / 'riscv-none-elf-g++'
objcopy = compiler_bin / 'riscv-none-elf-objcopy'
objdump = compiler_bin / 'riscv-none-elf-objdump'

compiler = f'{gcc} --entry _actual_start -T risc.ld -march=rv32izicsr -mabi=ilp32 -Wno-register -fno-exceptions -ffunction-sections -fdata-sections -Wl,--gc-sections -mcmodel=medany -O3 '

added_projects = []
source_files = []
includes = []
todo = []

def walk(s):
    for dirpath, dirnames, filenames in os.walk(s):
        for filename in filenames:
            if Path(filename).suffix not in ['.c', '.s', '.cpp', '.o']:
                # print('Skipping ' + filename)
                continue
            
            path = Path(dirpath) / filename
            print(f'Adding source file {path}')
            source_files.append(path)

def add(s):
    if s in added_projects:
        return
    added_projects.append(s)

    with open(Path('src') / s / 'deps.txt') as file:
        for line in file:
            todo.append(line.strip())
    
    if s.startswith('lib.'):
        includes.append(s)
    
    walk(Path('src') / s)

todo.append('lib.start')
todo.append(sys.argv[1])

while len(todo) > 0:
    add(todo.pop())

for folder in includes:
    path = Path('src') / folder
    compiler += f'-I {Path(path)} '

for file in source_files:
    compiler += f'{file} '

print(f'Compiler command: {compiler}')

r = os.system(compiler)
exit_if_error(r)

r = os.system(f"{objcopy} -O binary a.out program.bin")
exit_if_error(r)

# r = os.system(f"{objdump} -b binary -m riscv -D program.bin")
# exit_if_error(r)

with open('program.bin', 'rb') as f:
    data = f.read()

conv = base64.b64encode(data)
# print(conv)
with open('base64.txt', 'w') as f:
    f.write(str(conv)[2:-1])
