default:
	cross build --release --target arm-unknown-linux-musleabi
	ssh pi@rpi-printer.lan killall  /home/pi/serve-list-files || echo Not running
	scp ./target/arm-unknown-linux-musleabi/release/serve-list-files pi@rpi-printer.lan:/home/pi/
	ssh pi@rpi-printer.lan /home/pi/serve-list-files
run:
	cross run --target arm-unknown-linux-musleabi
