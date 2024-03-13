To run this project, 

in the side-scroller directory execute command cargo run



Features so far:
- wasd movement copied from Ldtk bevy tilemap example.

- Added new cursor system for moving player.
- ~~Currently the player snaps to the grid coords just as the prev wasd method.~~
- Added pathfinding to the player entity after clicking and dragging player using A* with pathfinding crates.


Future features:
- a line that follows the dragged cursor using the pathfinding system, circumventing walls.
- GUI around the window that displays action points for fox, end turn button, etc..
- Create enemies that follow path-finding rules, with specific traits too.

