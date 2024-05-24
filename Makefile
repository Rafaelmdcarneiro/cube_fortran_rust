# Must be using a KallistiOS environment with gccrs toolchain.
# Make sure to be using -m4-single FPU precision setting, including in KOS and GLdc.

TARGET = rustdc.elf

# Path to separate sh-elf toolchain with GFortran support.
FORT_CHAIN = /opt/toolchains/dc/sh-elf-fortran
LIBGFORTRAN_PATH = $(FORT_CHAIN)/sh-elf/lib/libgfortran.a

OBJS = rustdc.rox helpers.o ferris.o gcc.o claw.o

all: rm-elf $(TARGET)

include $(KOS_BASE)/Makefile.rules

%.rox: %.rs
	kos-cc -frust-incomplete-and-experimental-compiler-do-not-use $(CFLAGS) -c $< -o $@

%.o: %.f90
	$(FORT_CHAIN)/bin/sh-elf-gfortran $(CFLAGS) -c $< -o $@

clean: rm-elf
	-rm -f $(OBJS) helpers.mod ferris.vq gcc.vq claw.vq

rm-elf:
	-rm -f $(TARGET)

$(TARGET): $(OBJS)
	kos-cc -o $(TARGET) $(OBJS) -L$(KOS_BASE)/lib $(LIBGFORTRAN_PATH) -lGL -lm

ferris.o: ferris.vq gcc.vq claw.vq
	$(KOS_BASE)/utils/bin2o/bin2o ferris.vq ferris ferris.o
	$(KOS_BASE)/utils/bin2o/bin2o gcc.vq gcc gcc.o
	$(KOS_BASE)/utils/bin2o/bin2o claw.vq claw claw.o

ferris.vq: ferris.jpg gcc.jpg claw.jpg
	$(KOS_BASE)/utils/vqenc/vqenc -t -v ferris.jpg
	$(KOS_BASE)/utils/vqenc/vqenc -t -v gcc.jpg
	$(KOS_BASE)/utils/vqenc/vqenc -t -v claw.jpg

run: $(TARGET)
	$(KOS_LOADER) $(TARGET)

dist: $(TARGET)
	-rm -f $(OBJS)
	$(KOS_STRIP) $(TARGET)
