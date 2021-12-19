# Welcome to retro

This is the beginning of a RPG that I've had in the back of mind for close to 30 years now.  This is a library for
building an RPG game suitable either for CRPG's or PPRPG's.  It is far more crunchy and realistic than pretty much any
game you will have seen before.  Are you tired of:

- Hit Points as a health abstraction? (what do you mean I can't slit his throat while he is unconscious?)
- Combat feels the same, because there is not a lot of tactical depth, other than choosing what maneuver to do?
- No differentiation between how easy it is to hit, and what protection armor affords?
- Initiative systems that don't factor in not just when actions start, but when they finish?
- Weapons that all feel the same, because they only cover damage, range, cost, and maybe one or two other things?
- Wonder how a character can be a PhD in Xenobiology, but somehow wind up with Heavy Weapons skill?
- Characters who epitomize "fools rush in where angels fear to tread" because there is no mechanic (or desire) to 
  restrict character behavior?
- Personality traits that are just role playing fluff, and have no mechanics to penalize or reward their use?
- Characters who never get tired, because either your game's rules for encumberance are too complex, or doesn't even have
  them at all?
- Skill systems that treat all skills as equally easy to learn?
- Skill systems that do not distinguish between breadth and depth?

And that's just a few of the pain points that retro is trying to solve.  Another point retro is trying to solve may seem
a bit counter intuitive, given that it will stress realism and simulation.  Retro is designed to be small and modular,
so that you only need what you need.

Are you tried of reading through a wall of text to find the actual meaning or game functionality that you need?  Some
will say that good indexing or Referee screens can solve that, but I think it's not enough.  Plus, having to read 
through a wall of text to learn is not always fun.  I also hate having to read through a ton of game fiction to get at
the rules.  Call me old school, but games are games, and fiction is fiction.  The only place there should be fiction is
when describing examples of how the rules work.

I remember when complete games came with 64-96 pages of rules.  Nowadays, they come with 300-600 pages for the corebook!
I don't think it is impossible to create a game which has tons of crunch, and yet is also small both in page size, and
is keeping inside the RAM in your head.  A part of why I think this is possible, is because a lot of the rules are
in fact intuitive.

## PPRPG, CRPG or CARPG

If you aren't familiar with those acronyms, the first 2 are probably somewhat common and stand for:

- PPRPG: Paper and Pen Role Playing Game
- CRPG: Computer Roleplaying Game

The last one, CARPG is probably new, because AFAIK, I just made it up.  It stands for Computer Assisted Role Playing
Game.  In a PPRPG, you don't need anything electronic at all.  Just some paper, pens, the rules, dice, and possibly
maps, screens, and other gaming miscellania.  Retro isn't really meant for this kind of play style, though in theory,
you could print out the rules and character sheets.

In a CRPG, you are playing either a single player RPG, like Skyrim, or a multiplayer RPG, like World of Warcraft.  For
CRPG's, retro is designed to be a library that you can use to define your character.  It defines things like Attributes,
personality traits, damage classes, equipment building, etc, that you can use to help speed up the build of your game.

The primary goal of retro however, was as a CARPG.  So what is a CARPG?  A CARPG is sort of a hybrid between a CRPG and
a PPRPG.  You use a web browser as your "map and miniatures", and your character sheet and dice rolling can be done via
the browser as well.  If you have used Roll 20, then it's very much in the same vein.  The main difference is that you
the user can design your own game rules, expansions, adventures etc.  It's all pluggable and module.

## Design

retro will fit into a larger eco system which will include kubernetes services to set up everything needed to play from
the web:

- Web server backend for websockets, authentication and WebRTC
- Database(s) for storing game state, blogs and other collaboration content
- Front end SPA containing:
  - Character designer and sheets
  - Chat and video conference
  - 3d map with webgl
  - chatbot

Most likely, I will retrofit the existing khadga app for this.  In fact, I may wind up adding this to the multirepo in
the khadga project.

## Programming language consideration

I decided to write this in rust for a couple of reasons:

- I want to get better at rust
- Since it's rust, the shared library will be able to be used by C(++) applications
- I can also compile it down to webassembly for use in browsers
- It's blazing fast and it needs to be

It would be nice to do this in more popular languages to get more contributions, so I might wind up doing some pieces
in typescript (especially parts for tensorflow).