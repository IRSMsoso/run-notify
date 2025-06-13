# Run Notify

A rust program that runs another program, sending a notification to pushover when it completes (success or fail). Optionally, you can also shutdown your PC after the command completes.

## Install

With cargo. Clone the repo and `cargo install --path .`

## Usage

`run_notify <COMMAND>`

Anything after run_notify and it's flags is interpreted as the command and it's arguments, for example: `run_notify -s ls -l` is valid.

Optionally, `run_notify -s <COMMAND>` to shutdown your PC after the command completes.
