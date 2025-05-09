## Changes

* Fixed the bullet logic: Speedster bullets would tend to immediately dissapear though they seem to 
    be spawned and act as if they were visible
    - Adding an early return if statement for `Sprite` methods that regard to movement seem to have fixed it,
        bullets are now visible to the player.

## To-Do:

* Clean up imports, unecessary code, and comments

* Don't rely on escape characters to clear the screen, try and leverage `crossterm` to refresh the screen

* Make a final game animation

* Perhaps add sound
