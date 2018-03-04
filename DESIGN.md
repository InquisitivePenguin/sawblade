# Design patterns
Sawblade is designed to operate internally as a customizable
hierarchy: each 'layer' can control all of it's data, but
not the data of it's parent layer.

There are four 'layers' of abstraction in Sawblade: the Game, the World,
the Scene, and then the GameObject.

The Game controls the rendering, sound, input pipeline, etc. It also
runs the primary 'game loop'.

The World is an abstract representation of the game world.
It is notified when an event happens, and passes that event down
to the Scene or something else. It also controls (to some extent) the
final sprite location/texture list passed to the Game to be rendered.

The Scene is where most of your game's code will probably go.
It handles all events via it's `dispatch` function, which is overloadable.
It also contains Components which reduce boilerplate and handle the
majority of generic game events.

GameObjects are objects that populate a scene. They may or may not
have a graphical manifestation, with some acting purely as a background
object. Starting in 0.2, GameObjects will no longer directly return textures:
rather, they will have a list of components attached to them, of 
which may include a texture generator.

GameObjects can receive their texture from the `WorldState`, which
is a global state of the game that also acts a resource manager.