import os

def exit_if_error(r):
	if r != 0:
		print(f'Exit with code {r}')
		exit(r)

compiler = 'riscv-none-embed-gcc -T risc.ld -nostdlib -march=rv32i -mabi=ilp32 -ffunction-sections -fdata-sections -Wl,--gc-sections -O3 -mcmodel=medany '
for dirpath, dirnames, filenames in os.walk('src'):
	for filename in filenames:
		if filename[-2::] != '.c':
			print('Skipping ' + filename)
			continue
		file_fullpath = f'{dirpath}/{filename}'
		print('Adding ' + file_fullpath)
		compiler += f'"{file_fullpath}" '

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