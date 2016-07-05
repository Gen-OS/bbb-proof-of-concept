CHAIN = arm-none-eabi
ARCH = thumbv7a
TARGET = $(ARCH)-none-eabi

AS = $(CHAIN)-gcc -x assembler-with-cpp
ASFLAGS = -mcpu=cortex-a8 -mfloat-abi=hard -mfpu=neon

LD = $(CHAIN)-ld
LDFLAGS = -nostdlib

OBJCOPY = $(CHAIN)-objcopy

output := blackjack
libstage1 := target/$(TARGET)/debug/libstage1.a
linker_script := src/arch/$(ARCH)/linker.ld
assembly_source_files := $(wildcard src/arch/$(ARCH)/*.S)
assembly_object_files := $(patsubst src/arch/$(ARCH)/%.S, build/arch/$(ARCH)/%.o, $(assembly_source_files))

.PHONY: all copy clean run rust

all: $(output).boot

copy: $(output).boot
	cp $(output).boot tftpboot/$(output).boot 

clean:
	@xargo clean
	@rm -f *.elf
	@rm -f *.boot
	@rm -f *.dump
	@rm -rf build

rust:
	xargo build --target $(TARGET)

$(output).elf: rust $(libstage1) $(assembly_object_files) $(linker_script)
	$(LD) $(LDFLAGS) $(assembly_object_files) $(libstage1) -T $(linker_script) -o $(output).elf

$(output).boot: $(output).elf
	$(OBJCOPY) -O binary $< $@

# compile assembly files
build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.S src/arch/$(ARCH)/asm_common.h
	mkdir -p $(shell dirname $@)
	$(AS) $(ASFLAGS) $< -c -o $@