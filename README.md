# proctl

A CLI tool to bring individual CPUs online or offline on Linux.

Can be useful to reduce the power consumption of laptops when only a portion of the available
processing power would suffice to get the job done.

## Install

```bash
# build and install to /usr/local/bin

sudo make install

# uninstall binary
sudo make uninstall
```

## Usage

```bash
# Get the total number of CPUs that can be brought online at the same time on the system
proctl get-available

proctl get-online

proctl get-offline

# Adjust online CPUs so that 4 are online, bringing others offline.
# Requires elevated privileges due to the underlying write being performed.
proctl scale 4

# Bring all the CPUs online
proctl scale $(proctl get-available)
```

## How it works

This tool essentially provides a clean interface for reading and writing files located under `/sys/devices/system/cpu`.
