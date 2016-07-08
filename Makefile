CHAIN = arm-none-eabi
ARCH = thumbv7a
CPU = am335x
TARGET = $(ARCH)-none-eabi
BOARD = beaglebone-black

# This is slow and bad, but faster than tracing the assembly by hand.
AS = $(CHAIN)-gcc -x assembler-with-cpp
ASFLAGS = -mcpu=cortex-a8 -mfpu=neon -mfloat-abi=softfp

LD = $(CHAIN)-ld
LDFLAGS = -nostdlib --gc-sections

OBJCOPY = $(CHAIN)-objcopy

output := blackjack
libstage1 := target/$(TARGET)/debug/libstage1.a
linker_script := src/cpu/$(CPU)/linker.ld
assembly_source_files := $(wildcard src/arch/$(ARCH)/*.S)
assembly_object_files := $(patsubst src/arch/$(ARCH)/%.S, build/arch/$(ARCH)/%.o, $(assembly_source_files))

.PHONY: all clean run rust

all: $(output).bin

clean:
	@xargo clean
	@rm -f *.elf
	@rm -f *.boot
	@rm -f *.dump
	@rm -rf build

rust:
	xargo build --target $(TARGET) --features "board-$(BOARD)"

$(output).elf: rust $(libstage1) $(assembly_object_files) $(linker_script)
	$(LD) $(LDFLAGS) $(assembly_object_files) $(libstage1) -L src/arch/$(ARCH) -T $(linker_script) -o $(output).elf

$(output).bin: $(output).elf
	$(OBJCOPY) -O binary $< $@

# compile assembly files
build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.S src/arch/$(ARCH)/asm_common.h
	mkdir -p $(shell dirname $@)
	$(AS) $(ASFLAGS) $< -c -o $@
