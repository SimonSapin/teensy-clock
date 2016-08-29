SERIAL_DEVICE = /dev/ttyACM0

flash: hex
	teensy-loader-cli -w -s --mcu=mk20dx256 teensy3/main.hex

hex:
	@make --no-print-directory -C teensy3 NO_ARDUINO=1

cat:
	while sleep 1; do [ -e $(SERIAL_DEVICE) ] && cat $(SERIAL_DEVICE); done

timestamp:
	date +@%s > $(SERIAL_DEVICE)

clean:
	git clean -Xf
