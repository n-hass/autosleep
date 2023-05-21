#!/bin/bash

APP_PATH=""
if ls autosleep &> /dev/null; then
	chmod +x autosleep
	APP_PATH="autosleep"
fi
if ls target/release/autosleep &> /dev/null; then
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
mv -f $APP_PATH /usr/local/bin/autosleep

# make the directories needed
mkdir -p /etc/autosleep.d

# copy the service file
cp base/autosleep.service /usr/lib/systemd/system/autosleep.service

# copy the config file
cp base/autosleep.conf /etc/autosleep.d/autosleep.conf

# copy the checks dir
cp -r base/checks /etc/autosleep.d/checks
