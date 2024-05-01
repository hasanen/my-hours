# My Hours

![build & test](https://github.com/hasanen/my-hours/actions/workflows/rust.yml/badge.svg)

Goal of the project is to create command line app to combine tracked hours in tracking services and provide easy way to see if the monthly goal will be met or not.

Install

    cargo install my-hours

Then you can check help to see all available commands

    my-hours --help

## Usage

After building and setting up the alias:

```bash
$ my-hours --help
Usage: my-hours [OPTIONS] [COMMAND]

Commands:
  integrations  Manage integrations
  refresh       Refresh hours through integrations
  info          Show some basic info
  help          Print this message or the help of the given subcommand(s)

Options:
      --refresh  Refresh hours from integrations before printing them
  -h, --help     Print help information
```

### Usage with Toggl

Set up the integration to [Toggl](http://toggl.com/):

```bash
$ my-hours integrations setup toggl
Toggl API key:
deadbeefe1e7e59b53084173c2685f12
New toggle configuration saved!
$ my-hours integrations list
Enabled integrations:

Toggl, workspaces: Hours's workspace
```

Set your target hours:

```bash
$ my-hours
Updated monthly hours from integrations
What is your target daily hours for Website?
8
What is your target weekly hours for Website?
40
What is your target monthly hours for Website?
160

Updated monthly hours from integrations

 Project                   | Today | Current week / Daily AVG | Current month / Daily AVG | Target (day / week / month)
========================================================================================================================
 Test Client / Website     |       |  25h  0m /   8h 20m      |  25h  0m /   8h 20m       | 8h / 40h / 160h
---------------------------+-------+--------------------------+---------------------------+-----------------------------
 Total                     |       |  25h  0m /   8h 20m      |  25h  0m /   8h 20m       |

```

From now on, you can run `my-hours` to see the status. Targets can be updated in the settings, see `my-hours info`.
