SERIAL_DEVICE = /dev/ttyACM0

flash: hex
	teensy-loader-cli -w -s --mcu=mk20dx256 teensy3/main.hex

hex: rust
	@make --quiet -C teensy3 NO_ARDUINO=1

rust:
	cargo build --release

cat:
	while true; do [ -r $(SERIAL_DEVICE) ] && cat $(SERIAL_DEVICE); inotifywait -qq -e create -e attrib /dev; done

get:
	(sleep .01; echo g > $(SERIAL_DEVICE)) &
	head -n1 $(SERIAL_DEVICE)

sync:
	date --utc '+s%Y-%m-%d %H:%M:%S' > $(SERIAL_DEVICE)

clean:
	git clean -Xf
