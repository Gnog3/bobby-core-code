#!/usr/bin/env python3
import sys, os, shutil, base64, json, asyncio
from pathlib import Path
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection

CFLAGS='-O3 -ffunction-sections -fdata-sections -march=rv32izicsr -mabi=ilp32 -mcmodel=medany -T risc.ld -nostdlib -Wl,--gc-sections -D_GNU_SOURCE -Isrc/include'

LDFLAGS_BIN='-Wl,--unresolved-symbols=ignore-all,-q'

TEST_GCC = 'gcc'
GCC_PREFIX = '~/xpack-riscv-none-elf-gcc-15.2.0-1/bin'
GCC = f'{GCC_PREFIX}/riscv-none-elf-gcc'
OBJCOPY = f'{GCC_PREFIX}/riscv-none-elf-objcopy'
OBJDUMP = f'{GCC_PREFIX}/riscv-none-elf-objdump'
SUBASSEMBLY_FOLDER_PATH = '/mnt/c/Program Files (x86)/Steam/steamapps/common/Logic World/subassemblies'

for i in range(0, 1000):
    SUBASSEMBLY_NAME = 'Program' + str(i).zfill(3)
    path = f'{SUBASSEMBLY_FOLDER_PATH}/{SUBASSEMBLY_NAME}'
    if not os.path.isdir(path):
        break

AUTOROM = f'/mnt/c/Users/User/projects/c#/AutoROM/AutoROM/bin/Release/net8.0/linux-x64/publish/AutoROM --rom R8K32Bit --path "{SUBASSEMBLY_FOLDER_PATH}" --data out/boot/boot --name {SUBASSEMBLY_NAME}'

SHOW_COMMANDS = False

with open('config.json') as f:
    CONFIG = json.load(f)

async def run_command(cmd):
    if SHOW_COMMANDS:
        print(f"Running command: {cmd}")

    process = await asyncio.create_subprocess_shell(
        cmd,
        # stdout=asyncio.subprocess.PIPE,
        # stderr=asyncio.subprocess.PIPE
    )

    stdout, stderr = await process.communicate()

    if process.returncode != 0:
        print(f"Command failed with exit code {process.returncode}")
        sys.exit(process.returncode)

async def create_folder(path):
    await run_command(f'mkdir -p {path}')

async def clean():
    shutil.rmtree('out')

def get_obj_path(src):
    src = str(Path(src).with_suffix('.o'))
    return src.replace('src', 'out/obj', 1)

def get_dir(src):
    return str(Path(src).parent)

def get_file(elem):
    return elem[0] if type(elem) is list else elem

def get_flags(elem):
    return elem[1] if type(elem) is list else ''

def to_base64(b):
    return base64.b64encode(b).decode('ascii')

def extract_symbol_offsets(elf_file_path):
    symbol_offsets = {}
    
    with open(elf_file_path, 'rb') as f:
        elffile = ELFFile(f)
        
        # Find the .rela.text section
        rela_section = elffile.get_section_by_name('.rela.text')
        if not rela_section:
            raise ValueError("No .rela.text section found in the ELF file")
        
        # Get the symbol table
        symtab = elffile.get_section_by_name('.symtab')
        if not symtab:
            raise ValueError("No symbol table found in the ELF file")
        
        # Iterate through relocation entries
        for relocation in rela_section.iter_relocations():
            # Skip entries without symbol (like R_RISCV_RELAX)
            if relocation['r_info_sym'] == 0:
                continue
                
            # Get the symbol
            symbol = symtab.get_symbol(relocation['r_info_sym'])
            if not symbol:
                continue
                
            symbol_name = symbol.name
            if symbol_name[0] == '.':
                continue
            
            offset = relocation['r_offset']
            
            # Add to our mapping
            if symbol_name not in symbol_offsets:
                symbol_offsets[symbol_name] = []
            symbol_offsets[symbol_name].append(offset)
    
    print(f'Relocations of {elf_file_path}:')
    for symbol, offsets in symbol_offsets.items():
        print(f'{symbol}: {offsets}')

    return symbol_offsets

async def test_bobbyfs():
    valgrind = False
    sanitize = False
    fl = ''
    if len(sys.argv) > 2 and sys.argv[2] == 'valgrind':
        valgrind = True
    elif len(sys.argv) > 2 and sys.argv[2] == 'sanitize':
        fl = '-fsanitize=address,leak,undefined,pointer-compare,pointer-subtract'
    await create_folder('out')
    
    await run_command(f'{TEST_GCC} -g -DTESTING -DBOBBYFS_DYN_CTX {fl} src/lib/bobbyfs/tests/bobbyfs_tests.c src/lib/bobbyfs/bobbyfs.c src/lib/bobbyfs/tests/test_cursor.c src/lib/bobbyfs/tests/vector.c src/lib/bobbyfs/tests/test_ctx.c -lcheck -lm -o out/bobbyfs_tests')
    if len(sys.argv) > 2 and sys.argv[2] == 'valgrind':
        await run_command(f'valgrind --leak-check=full --track-origins=yes ./out/bobbyfs_tests')
    else:
        await run_command('./out/bobbyfs_tests')    

async def compile_file(file):
    src = get_file(file)
    dst = get_obj_path(src)
    flags = get_flags(file)
    await create_folder(get_dir(dst))
    print(f'Compiling {src}')
    await run_command(f'{GCC} -c {src} -o {dst} {CFLAGS} {flags}')

async def compile_boot2():
    files = CONFIG['boot']['files']
    
    elf = 'out/boot/boot.elf'
    result = 'out/boot/boot'
    binary_dump = 'out/boot/binary_dump.txt'

    sources = ' '.join(files)

    await create_folder(get_dir(elf))
    print(f'Compiling {elf}')
    await run_command(f'{GCC} {sources} -o {elf} {CFLAGS}')
    await run_command(f'{OBJDUMP} -m riscv -D {elf} > {binary_dump}')
    await run_command(f'{OBJCOPY} -O binary {elf} {result}')
    await run_command(AUTOROM)

def gen_bin_task(definition):
    async def task():
        name = definition['name']
        files = definition['files']

        elf = f'out/bin/{name}/{name}.elf'
        objcopied = f'out/bin/{name}/{name}.bin'
        result = f'out/bin/{name}/{name}'
        binary_dump = f'out/bin/{name}/binary_dump.txt'
        b64_file = f'out/bin/{name}/base64.txt'

        await create_folder(f'out/bin/{name}')

        tasks = [compile_file(file) for file in files]
        await asyncio.gather(*tasks)
        
        objs = ' '.join(map(lambda x: get_obj_path(get_file(x)), files))
        print(f'Linking {elf}')
        await run_command(f'{GCC} {objs} -o {elf} {LDFLAGS} -Wl,--entry=main {LDFLAGS_BIN}')
        offsets = extract_symbol_offsets(elf)

        await run_command(f'{OBJDUMP} -m riscv -D {elf} > {binary_dump}')
        await run_command(f'{OBJCOPY} -O binary {elf} {objcopied}')
        with open(objcopied, mode='rb') as file:
            objcopied_contents = file.read()
        
        bobbys_binary = bytearray()
        bobbys_binary += len(objcopied_contents).to_bytes(4, 'little') # length of the binary
        bobbys_binary += (0).to_bytes(4, 'little') # length of metadata
        bobbys_binary += objcopied_contents

        with open(result, mode='wb') as file:
            contents = file.write(bobbys_binary)

        b64 = to_base64(bobbys_binary)
        with open(b64_file, mode='w') as file:
            file.write(b64)
    
    return task

tasks = {
    'clean': clean,
    'test-bobbyfs': test_bobbyfs,
    'compile-boot': compile_boot2,
}

async def blob():
    for task in CONFIG['blob']['tasks']:
        tasks[task]()
    
    # Creating binary file with all files and folders wanted
    blob_binary = bytearray()

    # Folders

    blob_binary += len(CONFIG['blob']['folders']).to_bytes(4, 'little')

    for folder in CONFIG['blob']['folders']:
        encoded_name = folder.encode('ascii')
        blob_binary += len(encoded).to_bytes(4, 'little')

    blob_binary += len(CONFIG['blob']['files']).to_bytes(4, 'little')
    for file in CONFIG['blob']['files']:
        encoded_name = file.encode('ascii')
        with open(file, mode='rb') as file:
            contents = file.read()
        blob_binary += len(encoded_name).to_bytes(4, 'little')
        blob_binary += len(contents).to_bytes(4, 'little')
        blob_binary += encoded_name
        blob_binary += contents
    
    await create_folder(get_dir(CONFIG['blob']['target']))
    with open(CONFIG['blob']['target'], mode='wb') as file:
        file.write(blob_binary)
    

tasks['blob'] = blob

for b in CONFIG['bin']:
    task_name = f'compile-{b['name']}'
    tasks[task_name] = gen_bin_task(b)

def print_help():
    print('Usage: ./py.py <task>')
    print('Available tasks:')
    for task in tasks:
        print(f'  {task}')

async def main():
    if len(sys.argv) < 2:
        print_help()
        return

    task = sys.argv[1]
    if task in tasks:
        await tasks[task]()
    else:
        print_help()

if __name__ == '__main__':
    asyncio.run(main())
