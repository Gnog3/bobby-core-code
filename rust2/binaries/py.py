import os, sys, base64
from pathlib import Path
from elftools.elf.elffile import ELFFile

def run(s):
    r = os.system(s)
    if r != 0:
        print(f'Exit with code {r}')
        exit(r)

if len(sys.argv) < 2:
    print('Error: No binary name')
    exit(-1)

target = Path('target/riscv32i-unknown-none-elf')
gcc = 'riscv32-unknown-elf-gcc'
objcopy = 'riscv32-unknown-elf-objcopy'
objdump = 'riscv32-unknown-elf-objdump'
binary = target / 'release' / sys.argv[1]

run(f'xargo build --release --bin {sys.argv[1]} --target riscv32i-unknown-none-elf')

exports = []
imports = []
reladyn = []
with open(binary, 'rb') as f:
    elf = ELFFile(f)
    
    dynsym = elf.get_section_by_name('.dynsym')
    dynsym_addr = dynsym['sh_addr']
    print(f'dynsym addr: {hex(dynsym_addr)} (Code size: {dynsym_addr} bytes)')

    # Get relocation table
    rela_plt = elf.get_section_by_name('.rela.plt')

    for symbol in dynsym.iter_symbols():
        if symbol.name == 'main' or symbol.entry['st_info']['type'] != 'STT_FUNC':
            continue
        exports.append((symbol.name, symbol.entry['st_value']))
    try:
        for rel in rela_plt.iter_relocations():
            offset = rel['r_offset']
            symbol = dynsym.get_symbol(rel['r_info_sym'])
            name = symbol.name
            imports.append((name, offset))
    except AttributeError:
        pass

    try:
        rela_dyn = elf.get_section_by_name('.rela.dyn')
        for rel in rela_dyn.iter_relocations():
            offset = rel['r_offset']
            symbol = dynsym.get_symbol(rel['r_info_sym'])
            address = rel['r_addend']
            name = symbol.name
            print(f'{hex(offset)} should point to {hex(address)}')
            reladyn.append((offset, address))
    except AttributeError:
        pass

print('This binary exports the following functions:')
for name, address in exports:
    print(f"    Name: \"{name}\", Address: {hex(address)}")

print('This binary imports the following functions:')
for name, address in imports:
    print(f"    Name: \"{name}\", Address: {hex(address)}")

run(f'{objcopy} -O binary {binary} program.bin')

result = bytearray()
result += dynsym_addr.to_bytes(4, 'little')

with open('program.bin', 'rb') as f:
    data = f.read()

result += data[:dynsym_addr]

def map_to_bytes(map):
    bytes = bytearray()
    for name, address in map:
        encoded = name.encode('utf-8')
        length = len(encoded)
        bytes += length.to_bytes(4, 'little')
        bytes += encoded
        bytes += address.to_bytes(4, 'little')
    return bytes

def reladyn_to_bytes(reladyn):
    bytes = bytearray()
    for offset, address in reladyn:
        bytes += offset.to_bytes(4, 'little')
        bytes += address.to_bytes(4, 'little')
    return bytes

reladyn_bytes = reladyn_to_bytes(reladyn)
result += len(reladyn).to_bytes(4, 'little')
result += reladyn_bytes
export_bytes = map_to_bytes(exports)
result += len(export_bytes).to_bytes(4, 'little')
result += export_bytes
import_bytes = map_to_bytes(imports)
result += len(import_bytes).to_bytes(4, 'little')
result += import_bytes

with open(f'{sys.argv[1]}.exe', 'wb') as f:
    f.write(result)

print(f'Bobby\'s binary size: {len(result)}')

conv = base64.b64encode(result)
# print(conv)
with open('base64.txt', 'w') as f:
    f.write(str(conv)[2:-1])