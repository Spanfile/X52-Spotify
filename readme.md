# X52-Spotify

A Linux program written in Rust to display the currently playing Spotify track on the MFD of an X52/X52-Pro.

## Requirements

* [This driver](https://github.com/nirenjan/x52pro-linux). You'll most likely want to make the USB device accessible by a non-root account to avoid running this program as root. Sample udev rule [over here](10-x52.rules).
* Xorg

### Additionally for building (for `bindgen`):

* llvm-dev
* libclang-dev
* clang
