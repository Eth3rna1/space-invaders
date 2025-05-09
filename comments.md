## Changes

* Leveraged `crossterm` instead of ANSI escape codes to clear and refresh the screen

* BUG FIX: When implementing the windows api for reading keystrokes, I forgot about the `esc` and `p` game keys, they are now reimplemented

## To-Do:

* Clean up imports, unecessary code, and comments

* Make a final game animation

* Perhaps add sound
