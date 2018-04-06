# Overview
Sawblade uses a very extendable and flexible system, the
`Application -> Controller -> System -> Whatever` model.

If we want to represent each layer by what it does, it would look like
this: 

```
Application: Renders shapes and textures to the screen, manages the window ->
Controller: Updates the game's state, run Systems ->
System: Update the game's state -> 
Whatever: If you want, you can have the System activate subobjects or whatever
you want
```

Why do `Controller`s and `System`s both modify the state? To answer
that question, think of the `Controller` as a global state manager,
and the `System` as a individual component of the game that accomplishes
a certain task. Combined with the `HasComponent` trait for `GameObject`s,
you can easily make an ECS model with a `System`.

## So, what is the state?

The game's state is a struct that contains, completely,
all of the data the games stores during runtime. To save memory,
and to make Sawblade more efficient, only one copy of the State exists,
which is then modified by the `Controller`.

The state can be whatever struct you want: usually it contains
the 'scene' the game is in, as well as the entities in the scene. Sawblade
provides a `EntityMap` container for this.

The `Controller` and `System` have no type bounds for your state,
but to use certain Sawblade-default systems, like `UISystem<T: UIState>`, you'll
need to implement some traits for your state.

## That's simple enough. How can I build a game using this that will actually scale?

Great question!

First of all, Sawblade offers lots of resources to make the game development process easier. For example,
support for virtually any UI interface is built-in, and the `UIRenderer` can even auto-render to the screen for you!
Use of our `GameObject` system makes it surprisingly easy and manageable to build applications, and will probably look
familiar to you from other game engines. Keep in mind that though we don't have very many built-in functions
for `GameObject`s, you can extend your own classes based on that trait to do whatever you want:
one really great thing about Rust!

Sawblade's lightweight nature also helps it scale. Anything not needed isn't used, and
we try to make our backend stuff minimalistic and efficient. Lua scripting
_will_ probably make your game slower, but it's unknown by how much.

Compared to other game engines, Sawblade is surprisingly good at being able to build
games of any size.