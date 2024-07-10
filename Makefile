build:
	docker build -t card-reader .

find-usb:
	lsusb | grep "Card Reader" | awk '{print $2 "/" $4}' | sed 's/://'

rp3:
	docker run -t -i --privileged -v /dev/bus/usb/001/006:/dev/microm1 card-reader

chost:
	export DOCKER_HOST=ssh://rp3

rembuild:
	make chost
	make build
