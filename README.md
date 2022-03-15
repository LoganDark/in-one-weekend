# in-one-weekend

Super basic path tracer written using the [Ray Tracing in One Weekend](
https://raytracing.github.io/) guide. Initially written around October 2020,
with tweaks as late as May 2021, as of the initial commit.

Also my very first path tracer, so there are some weird quirks, but it's still
interactive, multithreaded, features (bad) image denoising thanks to OIDN, and
can make some nice stuff if you let it sit for a while.

Also HDR tonemapped. You can adjust exposure using the square bracket keys.

Keybinds:

- WASDQE: Move camera
- Arrow keys: Rotate camera
- Brackets: Adjust exposure
- Backslash: Reset exposure
- Comma/period: Adjust pixel scale
- X: Set 1x pixel scale
- Z: Set 8x pixel scale
- P: Toggle "progressive" mode (with it disabled, the program will only display
  1 spp)
- O: Toggle image denoising

Debug mode has parallelism disabled.

To run the app you have to obtain OIDN, set the OIDN_DIR environment variable to
its installation directory, and then make sure the program can load the
libraries on startup, usually by putting them in a search path, or, if on
Windows, copying the dlls to the same dir as the exe, because Windows is stupid.
