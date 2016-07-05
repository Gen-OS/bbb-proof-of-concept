MEMORY
{
    ram : ORIGIN = 0x402f0400, LENGTH = 0x1B400
}

SECTIONS
{
    .text :
    { 
        *(.text*)
        PROVIDE(__aeabi_unwind_cpp_pr0 = 0);
    } > ram
    .data : { *(.data*) } > ram
    .bss : { *(.bss*) } > ram
}

ENTRY(_start_from_ARM)