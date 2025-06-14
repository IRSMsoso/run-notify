# Run Notify

A rust program that runs another program, sending a notification to pushover when it completes (success or fail).
Optionally, you can also shut down your PC after the command completes.

## Install

With cargo. Clone the repo and `cargo install run_notify`

## Usage

`run_notify <COMMAND>`

Optionally, `run_notify -s <COMMAND>` to shut down your PC after the command completes.

Anything after run_notify and it's flags is interpreted as the command and it's arguments, for example:
`run_notify -s ls -l` is valid and would run ls with the -l flag. The -s flag is given to run_notify because it is
before the first non flag parameter.

## Configuration

You need to configure the app token and user key for pushover requests. Run the program once to generate the config
file.
