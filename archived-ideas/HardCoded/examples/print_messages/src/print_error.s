# Function void print_error(void)

.align 1

.section .text
print_error:
    # Prologue
    addi    sp, sp, -16 # Allocate space from stack
    sd      ra, 8(sp) # Store ra into stack
    sd      fp, 0(sp) # Store fp into Stack

    # Procedure call

    la      a0, print_error_string_01
    li      a1, FD_STDERR
    call    fputs

    # Epilogue
    ld      ra, 8(sp) # Load ra from stack
    ld      fp, 0(sp) # Load fp from stack
    addi    sp, sp, 16 # Deallocate space from stack
    ret

.section .rodata
    print_error_string_01: .string "ERROR: Lorem ipsum error."
