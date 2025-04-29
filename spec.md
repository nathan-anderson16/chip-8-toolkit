; Comment

-----REGISTERS-----
- $0 - $f: General-purpose (V0 - VF)
- $i: Index
- $d: Delay
- $s: Sound

-----INSTRUCTIONS-----
clear: Clear

ret: Subroutine return

jmp addr: Jump
- jo addr: Jump to addr + V0

call addr: Subroutine call

ske: Skip if equal
- vx, val
- vx, vy

skn: Skip if not equal
- vx, val (or val, vx)
- vx, vy

mov a, b: Move b into a
- vx, val
- vx, vy

- i, val
- vx, d
- d, vx
- s, vx
- No instruction for vx, s

add a, b: Add
- vx, val
- vx, vy

or a, b: Binary or

and a, b: Binary and

xor a, b: Binary xor

sub1 a, b: Subtract1

sub2 a, b: Subtract2

shr a, b: Shift right

shl a, b: Shift left

rand: Random

draw a, b, c: Draw

skk a: Skip if key

sknk a: Skip if not key

key a: Get key

font a: Font character

bcd a: BCD

store a: Store memory

load a: Load memory
