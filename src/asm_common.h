#pragma once

.altmacro
.syntax unified
.thumb

// return and conditional return (interworking)
.macro retx c=  ; bx\c& lr              ; .endm

// return and conditional return (non-interworking)
.macro ret c=   ; mov\c& pc, lr         ; .endm

// Unwinding stuff. 
#ifdef UNWIND
.macro .fn_start
	.fnstart
	.cantunwind
.endm
.macro .fn_end
	.fnend
.endm
#else
.macro .fn_start
.endm
.macro .fn_end
.endm
#endif

// ELF symbol types
.macro .sym_local label
	.local \label&
.endm

.macro .sym_global label
	.global \label&
.endm


.macro .sym_internal label
	.global \label&
	.hidden \label&
.endm

.macro .sym_exported label
	.global \label&
	.protected \label&
.endm

.macro .sym_imported label
	.global \label&
.endm

// Simple helpers to save retyping the same boilerplate for definitions.
.macro .done
.endm
.macro .donedone
	.purgem .done
	.macro .done
	.endm
.endm

.macro .armfun label, symtype=local
	.done
	.balign 4
	.arm
	.sym_\symtype& \label&
	.type   \label&, "function"
\label&:
	.fn_start
	.purgem .done
	.macro .done
		.fn_end
		.size \label&, . - \label&
		.donedone
		.thumb
	.endm
.endm

.macro .fun label, symtype=local
	.done
	.balign 4
	.thumb_func
	.sym_\symtype& \label&
	.type   \label&, "function"
\label&:
	.fn_start
	.purgem .done
	.macro .done
		.fn_end
		.size \label&, . - \label&
		.donedone
	.endm
.endm

.macro .sfun label, symtype=local
	.done
	.section .text.\label&, "ax", %progbits
	.balign 4
	.thumb_func
	.sym_\symtype& \label&
	.type   \label&, "function"
\label&:
	.fn_start
	.purgem .done
	.macro .done
		.fn_end
		.size \label&, . - \label&
		.donedone
	.endm
.endm

.macro .var label, symtype=local
	.done
	.balign 4
	.sym_\symtype& \label&
	.type   \label&, "object"
\label&:
	.purgem .done
	.macro .done
		.size \label&, . - \label&
		.donedone
	.endm
.endm