ARCHITECTURE=thumbv7em-none-eabihf

PROJECT_NAME=led-roulette
BUILD_PATH=target/${ARCHITECTURE}/debug/${PROJECT_NAME}

build:
	xargo build --target ${ARCHITECTURE}

gdb:
	arm-none-eabi-gdb -q ${BUILD_PATH}
