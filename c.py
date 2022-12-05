from time import sleep
import os, sys

from pathlib import Path
if len(sys.argv) < 2:
	print('error: no arguments')
	exit()
def exit_if_error(r):
	if r != 0:
		print(f'Exit with code {r}')
		exit(r)



compiler = 'riscv-none-embed-gcc -T risc.ld -nostartfiles -march=rv32i -mabi=ilp32 -ffunction-sections -fdata-sections -Wl,--gc-sections -mcmodel=medany -O3 '

def highLoadTest(time):
	print('Performing tests...')
	try:
		while True:
			walk(15)
	except:
		print('Test failed!!!')
	
def walk(s):
	global compiler

	for dirpath, dirnames, filenames in os.walk(s):
		for filename in filenames:
			if Path(filename).suffix not in ['.c', '.s', '.cpp']:
				# print('Skipping ' + filename)
				continue
			file_fullpath = f'{dirpath}/{filename}'

			print('Adding ' + file_fullpath + ' (folder: ' + dirpath + ')')
			compiler += f'"{file_fullpath}" '

walk('src/lib')
walk('src/' + sys.argv[1])

r = os.system(compiler)
exit_if_error(r)

r = os.system("riscv-none-embed-objcopy -O binary a.out program.bin")
exit_if_error(r)

r = os.system("riscv-none-embed-objdump -b binary -m riscv -D program.bin")
exit_if_error(r)

with open('program.bin', 'rb') as f:
	data = f.read()

ROM = ["", "", "", ""]

c = 3

for b in data:
	h = hex(b)[2:].upper()
	if len(h) == 1:
		h = '0' + h
	ROM[c] += h

	if c == 0:
		c = 3
	else:
		c -= 1

for i in range(4):
	print(f'{i}: {ROM[i]}')
