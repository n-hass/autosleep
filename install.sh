cargo build --release

# copy the binary
cp target/release/autosleep /usr/local/bin/autosleep

# make the directories needed
mkdir -p /etc/autosleep.d

# copy the service file
cp base/autosleep.service /usr/lib/systemd/system/autosleep.service

# copy the config file
cp base/autosleep.conf /etc/autosleep.d/autosleep.conf

# copy the checks dir
cp -r base/checks /etc/autosleep.d/checks