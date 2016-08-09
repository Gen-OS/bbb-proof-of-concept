/* src/cpu/am335x/linker.ld
 * Copyright 2016 Alexis Williams
 *
 * Licensed under the MIT License <http://opensource.org/licenses/MIT>.
 */

INCLUDE arch_universal.ld

MEMORY
{
    internal_ram : ORIGIN = 0x402f0400, LENGTH = 0x9600
    memory_cont  : ORIGIN = 0x40300000, LENGTH = 0x10000
}

/* Bootloader expects one segment, makes the ELF reflect that. */
PHDRS {
	image PT_LOAD;
}

SECTIONS
{
    . = 0x402f0400;

    .text . : { 
        *( 
            .text.entrypoint
            .text._start
            .text.vectors 
        )
        *( .text .text* )
    } > internal_ram :image

    .rodata (NOLOAD) : {
        *( .rodata .rodata* )
    } > internal_ram :image
    
    .data (NOLOAD) : {
		*( .data .data.* )
	} > internal_ram :image
    /* End of output here. */

	/* uninitialized shared RAM */
	.shared (NOLOAD) : {
        *( .shared .shared.* )
    } > memory_cont :NONE

	.bss : {
		__bss_start = .;
		*( .bss .bss.* COMMON )
		__bss_end = .;
	} > internal_ram :NONE

	/* uninitialized private RAM */
	.noinit (NOLOAD) : {
        *( .noinit .noinit.* )
    } > internal_ram :NONE

    /DISCARD/ : {
        *(.ARM.extab* .gnu.linkonce.armextab.*) 
        *(.ARM.exidx*)
    }
}
