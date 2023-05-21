#!/bin/bash

APP_PATH=""
if ls autosleep &> /dev/null; then
	chmod +x autosleep
	APP_PATH="autosleep"
elif ls target/release/autosleep &> /dev/null; then
	APP_PATH="target/release/autosleep"
else
	cargo build --release && APP_PATH="target/release/autosleep"
fi

if [ -z "$APP_PATH" ]; then
	echo "no autosleep binary found"
	exit 1
fi

for i in $(ls base/checks); do
	chmod +x base/checks/$i
done

# move the binary
sudo mv -f $APP_PATH /usr/local/bin/autosleep

# make the directories needed
sudo mkdir -p /etc/autosleep.d

# copy the service file
sudo cp base/autosleep.service /usr/lib/systemd/system/autosleep.service

# copy the config file
sudo cp base/autosleep.conf /etc/autosleep.d/autosleep.conf

# copy the checks dir
sudo cp -r base/checks /etc/autosleep.d/checks
