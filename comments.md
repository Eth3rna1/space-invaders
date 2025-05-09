## Changes

* Fixed a bug in the 2nd stage of the speedster.
    - When the speedster is as far as up or down as it can go,
      using the `move_right()` or `move_left()` methods don't necessarily give the sprite
      the desired movement pattern. Instead I made use of the `move_relative_x()` method
      to forcefully move the sprite in the desired direction
        - `move_left()` -> `move_relative_x()`
        - `move_right()` -> `move_relative_x()`
    

## To-Do:

* Fix the bullet logic: Speedster bullets tend to immediately dissapear though they seem to 
    be spawned and act as if they were visible

* Clean up imports, unecessary code, and comments

* Don't rely on escape characters to clear the screen, try and leverage `crossterm` to refresh the screen

* Make a final game animation

* Fix speedster bullets

* Perhaps add sound
