# protean-service

protean-service aims to be a Virtual Table Top Role Playing Game engine powered by bevy.  Beyond just a virtual 
table top application, it also aims to help with collaboration, a way to tell a visually and audially enhanced
story.

The first iteration of the service will also contain a playable story and helpers for solo style play.  It uses
the protean engine which is a role playing game engine designed for both table top and computer role playing  games.
The name protean comes from the Greek God Proteus, and means being able to taje many forms.

The service will be composed of several pieces:

- backend: for multiplayer, allows players to connect and maintains shared state of the world
- frontend: a bevy game engine which the player will interact with to control the character and get info on the world
- protean-lib: the base game engine that the frontend will use.  It contains the kernel to which plugins attach
- protean-plugins: adds new rules

## protean game engine

The heart of protean-service will be protean-lib that actually defines the base system.  It is designed to be modular, so
that it is very minimal but can be extended easily.  All the "rules" will actually be defined in plugins, which add or
implement base traits.  It will also be dynamic, and allow hot plugging via webassembly modules.
