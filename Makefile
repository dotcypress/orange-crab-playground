MODULE = Blinky
BUILD_DIR = target/bitstream
CONSTRAINTS = src/main/scala/playground/bsp/orangecrab.lpf

all: elaborate bitstream

elaborate: 
	sbt --supershell=never "runMain playground.$(MODULE)"

bitstream: 
	cd $(BUILD_DIR) && \
	yosys -q -p 'synth_ecp5 -top $(MODULE) -json $(MODULE).json' $(MODULE).v && \
	nextpnr-ecp5 --25k --package CSFBGA285 --json $(MODULE).json --textcfg $(MODULE).config --lpf ../../$(CONSTRAINTS) && \
	ecppack --compress --input $(MODULE).config --bit $(MODULE).dfu && \
	dfu-suffix -v 1209 -p 5af0 -a $(MODULE).dfu

prog:
	dfu-util -D $(BUILD_DIR)/$(MODULE).dfu

clean:
	sbt clean --supershell=never
	rm -rf $(BUILD_DIR)

.SECONDARY:
.PHONY: all bitstream build clean elaborate flash prog
