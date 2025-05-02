## Changes

* Created `events` to stage 3 end game phases
    - `SnowEvent`

* Renamed the `won` variable to `game_won`

* Included the comments file in the commits

* Made the `Sprite` internal variable, `coordinates`, private, created `coordinates()` and `coordinates_mut()`

* Made the `Sprite` internal variable, `velocity`, private, created `velocity()` and `set_velocity()`

* Made the `Sprite` internal variable, `engine`, private, created `engine()`

* Fixed a bug with the `move_down()` method in `Sprite`.

* Centered end game messages. Better aesthetic.

* Created the `rand_num()` function in `utils.rs`

* Created the `Obstacle` entity
    - New constants
        > `OBSTACLE_WAIT_TIME`
        > `OBSTACLE_SPEED`

* Formatted code

* Added the `coordinates()` method to `Shooter`

* Added **3** phases to the final stage of the speedster
    - Snowing phase
    - Falling blocks phase
    - Zig Zag phase
    + Implemented custom messages for each phase

## To-Do:

* Clean up imports, unecessary code, and comments

* Make `Speedster` have three lives and make it harder for the player to kill by making `Speedster` move in fun ways.

* Don't rely on escape characters to clear the screen, try and leverage `crossterm` to refresh the screen

* Make a final game animation
