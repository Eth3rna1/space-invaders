## Changes

* Got rid of keyboard input delay for windos while maintaining OS compatability.
    - Used the `winapi` crate

* Included the `winapi` crate into the project

## To-Do:

* Clean up imports, unecessary code, and comments

* Make `Speedster` have three lives and make it harder for the player to kill by making `Speedster` move in fun ways.

* Don't rely on escape characters to clear the screen, try and leverage `crossterm` to refresh the screen

* Make a final game animation

* Fix speedster bullets

* Perhaps add sound
