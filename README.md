To run this project, 

in the side-scroller directory execute command cargo run



Features so far:
- wasd movement copied from Ldtk bevy tilemap example.

- Added new cursor system for moving player.
- Currently the player snaps to the grid coords just as the prev wasd method.


Issues I'll need to address: 
- the player entity can be clicked and dragged directly to the destination. Need limits on distance traveled.
- The distance traveled should have a path-drawing system that draws a line following the rules of movement (going around walls)


Future features:
- path-finding with drawn line for visual of possible paths for fox.
- GUI around the window that displays action points for fox, end turn button, etc..
- Create enemies that follow path-finding rules, with specific traits too.
- 

