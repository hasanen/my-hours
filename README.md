# My Hours

![build & test](https://github.com/hasanen/my-hours/actions/workflows/rust.yml/badge.svg) [![codecov](https://codecov.io/gh/hasanen/my-hours/branch/main/graph/badge.svg?token=KJC3MNG6Z6)](https://codecov.io/gh/hasanen/my-hours)

Goal of the project is to create command line app to combine tracked hours in tracking services and provide easy way to see if the monthly goal will be met or not.

Until this has been published, you can do a release build after cloning the repository and setting an alias

    cargo build --release
    alias hours="~/PATH/target/release/my-hours

Then you can check help to see all available commands

    hours --help

## Tests

- update snapshots `TRYCMD=overwrite cargo test`
