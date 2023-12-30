# megalock: X11 xlock replacement

This currently is a substrate for a minimal xlock replacement in Rust. It is only functional with X11, but performs the following functions:

- Blanks the entire screen regardless of monitor configuration
- Grabs the keyboard and pointing devices
- Buffers all input into a password
- Performs all password checking through PAM
- Upon pressing enter, verifies that password
    - On success, the program exits
    - On failure, the program clears the password field and continues
- Additionally, the following environment variables are respected:
    - `TRACE=1`: Enable Trace (lowest) level debugging; includes passwords in plain text
    - `DEBUG=1`: Enable Debug level logging -- most status changes, but no raw data
    - `EXIT_TRAP=n`: Forcefully terminate the program after `n` seconds

It performs no indication of events while displaying the lock screen. This is
being worked on now.

Events are all synthetic, which are then translated to X11 calls in a dedicated thread. This should allow for easy porting to wayland later without breaking the X11 contract, allowing `megalock` to work with both systems natively.

Optimized binary size is about 730k, which was a nice discovery. We'll see if that sticks.

## Installation

```
cargo install megalock
```

Debian and RedHat packages will come soon!

### PAM configuration

If you use `megalock`, be sure to put [contrib/megalock](contrib/megalock) in `/etc/pam.d` so that megalock can use the PAM subsystem effectively. `megalock` may not work on OpenBSD.

## Thanks

Special thanks to Michael Stapelberg, who wrote `i3lock`, I spent a lot of time reading it while learning how to write this program.

## Author

Erik Hollensbe <git@hollensbe.org>

## License

MIT
