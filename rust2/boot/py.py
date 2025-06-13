import os, sys, base64
from pathlib import Path

def run(s):
    r = os.system(s)
    if r != 0:
        print(f'Exit with code {r}')
        exit(r)

compiler_bin = Path('../../compiler/bin')
target = Path('target/riscv32i-unknown-none-elf')
gcc = 'riscv32-unknown-elf-gcc'
objcopy = 'riscv32-unknown-elf-objcopy'
objdump = 'riscv32-unknown-elf-objdump'
binary = target / 'release' / 'boot'

os.environ['CC'] = str(compiler_bin)
os.environ['CFLAGS'] = '-march=rv32izicsr -mabi=ilp32 -ffunction-sections -fdata-sections -mcmodel=medany -O3'


run('cargo build --release')

run(f'{objcopy} -O binary {binary} program.bin')

with open('program.bin', 'rb') as f:
    data = f.read()

conv = base64.b64encode(data)
# print(conv)
with open('base64.txt', 'w') as f:
    f.write(str(conv)[2:-1])