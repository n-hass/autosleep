# About
This is a really lightweight program intended to run daemonised on Linux servers/PCs with systemd. It will perform configurable checks for activity on the system and, if the system has been idle long enough, will suspend the system.

Your defined activity checks will all be run together every set number of seconds defined in your configuration file. They will all be checked iteratively but will short circuit; if one check returns that the system is active, the remaining checks will be skipped.

# Installation
1. Clone the repo
2. Build with `./build.sh`
3. Install with `sudo ./install.sh`

This will: build the binary application and move it to `/usr/local/bin`, install the systemd service and the configuration files with a couple demonstrative checks in the directory `/etc/autosleep.d/checks`.

# Configuration
See the [wiki](https://github.com/n-hass/autosleep/wiki/Configuration) for info on configuration.

# Usage
After installation, start and enable the systemd service with:

`systemctl enable --now autosleep.service`

By default, the program will log basic info about the checks to the system journal so you can check what it's doing at runtime.

# To-do
- [x] 'Users' class checks
  - [ ] Check compatibility across different distributions
  - [ ] Rewrite to use a crate like `rust-psutil` once their features are implemented
- [x] 'Command' class checks
- [ ] More types of activity checks
  - [ ] Network (incoming and outgoing connections)
  - [ ] Process-based
  - [ ] X11 idle detection
- [ ] Scheduling a wake-up
- [ ] Documentation
- [ ] Unit tests
- [x] Releases contain builds
- [ ] Publish on package managers
