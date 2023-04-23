# os/src/entry.asm
.globl _start
.globl boot_stack

.section .text.entry
_start:
    la sp, boot_stack_top
    call rust_main

.section .bss.stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: