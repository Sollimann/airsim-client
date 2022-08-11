.PHONY: airsim-up airsim-down

export LD_LIBRARY_PATH=/usr/local/lib
export CLOUDSDK_PYTHON := /usr/bin/python2

airsim-up:
		sudo xhost local:root && docker-compose up --build airsim

airsim-down:
		docker-compose down
