import os, sys, base64
from pathlib import Path
from elftools.elf.elffile import ELFFile

def run(s):
    r = os.system(s)
    if r != 0:
        print(f'Exit with code {r}')
        exit(r)

compiler_bin = Path('../compiler/bin')
target = Path('target/riscv32i-unknown-none-elf')
gcc = compiler_bin / 'riscv-none-elf-gcc'
objcopy = compiler_bin / 'riscv-none-elf-objcopy'
objdump = compiler_bin / 'riscv-none-elf-objdump'
binary = target / 'release' / 'hello-world'

os.environ['CC'] = str(compiler_bin)
os.environ['CFLAGS'] = '-march=rv32izicsr -mabi=ilp32 -ffunction-sections -fdata-sections -mcmodel=medany -O3'

r = 1
while r != 0:
    r = os.system('cargo build --release')

with open(binary, 'rb') as f:
    elf = ELFFile(f)
    
    # Get relocation table
    rela_plt = elf.get_section_by_name('.rela.plt')
    dynsym = elf.get_section_by_name('.dynsym')
    
    for rel in rela_plt.iter_relocations():
        print(rel)
        offset = rel['r_offset']
        symbol = dynsym.get_symbol(rel['r_info_sym'])
        name = symbol.name
        print(f"Offset in .got.plt: {hex(offset)} -> Function: {name}")

run(f'{objcopy} -O binary {binary} program.bin')

with open('program.bin', 'rb') as f:
    data = f.read()

conv = base64.b64encode(data)
# print(conv)
with open('base64.txt', 'w') as f:
    f.write(str(conv)[2:-1])