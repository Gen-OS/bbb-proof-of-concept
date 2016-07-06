#include "asm_common.h"

/// ## Allocate the stack.
.pushsection .noinit.stack, "aw", %nobits
    .balign 8
.var stack
    .skip  4096
.done
.set stack_top, .

/// ## ARM entrypoint
/// do nothing but jump over into (potentially) Thumb real entrypoint.
.section .text.entrypoint, "ax", %progbits
.armfun _start_from_ARM, global
    ldr     pc, = vectors
.done
.pool


.section .text, "ax", %progbits

/// ## Exception handling
///    Fake the simplicity of the Cortex-Mx series on the Cortex-Ax, at least for now.
///
///    In handler mode, the following modes are available:
///    0b11111 | system mode (same stack as thread mode)
///    0b10011 | supervisor mode (separate stack, not supported right now)
///
#define MODE_USR 0b10000    // thread mode, unprivileged
#define MODE_THR 0b11111    // thread mode, privileged
#define MODE_HND 0b11111    // handler mode (always privileged)

.macro henter lradjust
 .if \lradjust&
    sub     lr, \lradjust&  // fix return address
 .endif
    srsfd   sp!, MODE_HND   // save { lr, spsr } to handler stack
    cpsid   ia, MODE_HND    // change to handler mode, mask async exceptions
.endm

.macro hret
    rfefd   sp!             // pop { pc, cpsr }
.endm

// Note: when calling C code, make sure { r0-r3, r12, lr } is saved/restored
.section .text.vectors
    .balign 32
.fun _start, exported
.fun vectors
    bl.w    init
    b.w     exc_undef
    b.w     exc_syscall
    b.w     exc_iabort      // instruction-side fault and debug trap
    b.w     exc_dabort      // data-side fault (includes cache maintainance)
    b.w     .               // reserved for hypervisor (n/a on cortex-a8)
    b.w     exc_irq
    b.w     .               // FIQ is not available, will never be used

.fun exc_undef
    henter  0
    push    { r0-r3, r12, lr }
    mov     r0, sp
    blx     undef_dispatcher
    pop     { r0-r3, r12, lr }
    hret

.fun exc_iabort
    henter  4
    push    { r0-r3, r12, lr }
    mov     r0, sp
    blx     iabort_dispatcher
    pop     { r0-r3, r12, lr }
    hret

.fun exc_dabort
    henter  8
    push    { r0-r3, r12, lr }
    mov     r0, sp
    blx     dabort_dispatcher
    pop     { r0-r3, r12, lr }
    hret

.fun exc_irq
    henter  4
    push    { r0-r3, r12, lr }
    mov     r0, sp
    blx     irq_dispatcher
    pop     { r0-r3, r12, lr }
    hret

.fun exc_syscall
    henter  0
    push    { lr }
    // Syscalls are easy: save only lr and dispatch based on r12.
    // Register arguments r0-r3 and return value are transparently passed
    // through to/from the syscall handler.
    // Saving lr is not needed if you have separate handler/process stacks
    // and syscalls are only made by processes and never in handler mode.
    pop     { lr }
    hret

// Actual processor initialization.
.sfun init
    // Execution arrives here by bl/blx from reset vector, thereby handing 
    // us the location and thumbness of the exception vectors.
    subs    r4, lr, 4       // Vector table base, bit 0 = thumbness

    cpsid   ia, MODE_THR    // thread mode, mask all async exceptions

    ldr     sp, = stack_top

    // initialize Neon/VFP (since compiler may use it at any time)
    mov     r5, -1
    mcr     p15, 0, r5, c1, c0, 2   // allow coprocessor access
    mov     r5, ( 1 << 30 )
    vmsr    fpexc, r5               // enable Neon/VFP
    mov     r5, ( 3 << 24 )
    vmsr    fpscr, r5               // VFP "RunFast" mode

    // initialize system control register
    mov     r5, (3 << 11)           // only branch prediction and icache
    bfi     r5, r4, 30, 1           // configure arm/thumb-mode exceptions
    mcr     p15, 0, r5, c1, c0, 0   // system control register

    // initialize exception vector base
    bics    r4, r4, 1               // clear thumb-bit
    mcr     p15, 0, r4, c12, c0, 0  // vector base register

    cpsie   a                       // unmask async aborts

    // r0-r3 have been preserved, so it's still possible to pass arguments
    bl      main

    // If main returns, go into infinite wfi-loop.  This means that purely
    // irq-driven apps only have to do their initialization in main.
.fun idle
    bl      wfi
    b       idle

.fun wfi, exported
    dsb
    wfi
    //b       irq_dispatcher

.fun abort, exported // Spinlock the processor
    b       abort
.done