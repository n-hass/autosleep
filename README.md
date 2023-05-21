# About
This is a really lightweight program intended to run daemonised as a systemd service. It will perform configurable checks for activity on the system and, if the system has been idle long enough, will suspend the system.

It's more or less a port from Python to Rust of the [autosuspend](https://github.com/languitar/autosuspend/tree/main) AUR package by [languitar](https://github.com/languitar). I was having trouble with some of its python dependencies, thought aspects of it could be simplified for my limited use case and wanted to exercise my Rust programming, so here we are!

It's behaviour is identical for the subset of its features that I have currently implemented.

Your defined activity checks will all be run together every set number of seconds defined in your configuration file. They will all be checked iteratively and will short circuit; if one check returns that the system is active, the remaining checks will be skipped.

# Installation
1. Clone this repo
2. Build with `cargo build --release`
3. Run `sudo install.sh`
This will: build the binary application and move it to `/usr/local/bin`, install the systemd service and the configuration files with a couple demonstrative checks in the directory `/etc/autosleep.d/checks`.

# Configuration
Edit the file `/etc/autosleep.d/autosleep.conf` for changing the configuration. The configuration is almost a strict subset of [autosuspend's](https://autosuspend.readthedocs.io/en/v4.3.1/)

The config file is very basic right now as the program itself is still quite small in terms of features. The file is commented to demonstrate what values a field can take.

Importantly, you can change where the app logs to (a file, stdout or the systemd journal), or disable logging, by changing the `log_target` value.

The `interval` value specifies how often to perform activity checks. Decreasing this value will **increase** the resolution of the data that autosleep will get by checking for activity more often, at the cost of the slight performance hit of running the checks. In most cases, this is neglible, but adjust to your liking. 

The program will never suspend your system while you are using it, as long as your checks are configured to detect *your* definition of usage!

## Adding checks
Currently **2** types of activity checks are implemented: `Users` and `Command`.

`Users` checks can be added by adding a section to the config file and specifying `class = Users`. The `names`, `terminals` and `hosts` are comma seperated values that will be regex matched, so using `".*"` will match any. Only `names` is required, the other fields will default to `".*"`.

`Command` checks will run a command (or shell script) and act on its return value; if the command/script returns a 0 exit code, it will be interpreted as the system is active, with non-zero exit codes indicating the system is inactive. Using this, you can implement your own check as a bash script, put it in the `/etc/autosleep.d/checks` directory, add it to the config file and *voilà*.

# Usage
After installation, start and enable the systemd service with `systemctl enable --now autosleep.service`. By default, the program will log basic info about the checks to the system journal so you can check what its doing at runtime there.

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
- [ ] Publish on package managers
