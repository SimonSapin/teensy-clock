flash: hex
	teensy-loader-cli -w --mcu=mk20dx256 teensy3/main.hex

hex:
	@make --no-print-directory -C teensy3 NO_ARDUINO=1
