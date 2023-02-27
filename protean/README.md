# Welcome to protean

This is the beginning of a RPG that I've had in the back of mind for close to 30 years now.  This is a library for
building an RPG game suitable either for CRPG's or PPRPG's.  It is far more crunchy and realistic than pretty much any
game you will have seen before.  Are you tired of:

- Hit Points as a health abstraction? (what do you mean I can't slit his throat while he is unconscious?)
- Combat feeling the same, because there is no interplay of speed, power, and maneuverability in the actions you choose?
- No differentiation between how easy it is to hit, and what protection armor affords?
- Everyone gets (at least) one turn per round, even though some beings are supernaturally fast?
- Initiative systems that don't factor in not just when actions start, but when they finish?
- Weapons that all feel the same, because they only cover damage, range, cost, and maybe one or two other things?
- How a character can be a PhD in Xenobiology, but somehow wind up with Heavy Weapons skill?
- Characters who epitomize "fools rush in where angels fear to tread" because there is no mechanic (or desire) to 
  restrict character behavior?
- Personality traits that are just role playing fluff, and have no mechanics to penalize or reward their use?
- Characters who never get tired, because either your game's rules for encumberance are too complex, or doesn't have
  them at all?
- Skill systems that treat all skills as equally easy to learn?
- Skill systems that do not distinguish between breadth and depth?
- Systems that encourage min-maxing by having cutoffs (eg, a 10 is as good as a 13, because they both give you +2)?

And that's just a few of the pain points that protean is trying to solve.  Another point protean is trying to solve may seem
a bit counter intuitive, given that it will stress realism and simulation.  protean is designed to be small and modular,
so that you only need what you need.

Are you tired of reading through a wall of text to find the actual meaning or game functionality that you need?  Some
will say that good indexing or Referee screens can solve that, but I think it's not enough.  Plus, having to read 
through a wall of text to learn is not always fun.  I also hate having to read through a ton of game fiction to get at
the rules.  Call me old school, but games are games, and fiction is fiction.  The only place there should be fiction is
when describing examples of how the rules work.

I remember when complete games came with 64-96 pages of rules.  Nowadays, they come with 300-600 pages for the corebook!
I don't think it is impossible to create a game which has tons of crunch, but is also small in both page size and the
knowledge to keep in your head.  A part of why I think this is possible is because a lot of the rules are in fact
intuitive.  Also, because there are many game attributes, you need less special one-off rules, or unique feats to cover
the same game rules.  In other words, while the game might initially appear to have way more datum than you need, this
is in fact a good thing, because it requires less special rules.

## TTRPG, CRPG or CARPG

If you aren't familiar with those acronyms, the first 2 are probably somewhat common and stand for:

- TTRPG: Table Top Role Playing Game (AKA PPRPG or Pen and Paper Role Playing Games)
- CRPG: Computer Roleplaying Game
- CARPG: Computer Assisted Roleplaying Game

The last one, CARPG, is probably new because AFAIK, I just made it up.  It stands for Computer Assisted Role Playing
Game.  In a TTRPG, you don't need anything electronic at all.  Just some paper, pens, the rules, dice, and possibly
maps, screens, and other gaming miscellania.  protean isn't really meant for this kind of play style, though in theory,
you could print out the rules and character sheets.

In a CRPG, you are playing either a single player RPG, like Skyrim, or a multiplayer RPG, like World of Warcraft.  For
CRPG's, protean is designed to be a library that you can use to define your character.  It defines things like Attributes,
personality traits, damage classes, equipment building, etc, that you can use to help speed up the build of your game.

The primary mode of protean however, is as a CARPG.  So what is a CARPG?  A CARPG is sort of a hybrid between a CRPG and a
PPRPG.  You use a web browser or app as your "map and miniatures", and your character sheet and dice rolling can be done
via the app as well.  If you have used Roll 20, then it's very much in the same vein.  The main difference is that you
the user can design your own game rules, expansions, adventures etc.  It's all pluggable and modular.

## Solo playing

Many gamers are not able, or even desire, to play with groups of other gamers.  Even in this day and age of virtual table top
systems that allow gamers to play together remotely, there is still a not insignificant niche of players who do solo
playing.  The Mythic game system is a great example, and it has been a best seller on drivethrurpg.com for several weeks now.

protean also caters to this crowd by making it easier to do solo playing.  This includes tables

## Design

protean will fit into a larger eco system which will include kubernetes services to set up everything needed to play from
the web:

- Web server backend for websockets, authentication and WebRTC
- Database(s) for storing game state, blogs and other collaboration content
  - Offline database for historical data (eg, collaboration content)
  - Near real-time database for in-game data (eg, character position, status, chats)
- Front end SPA containing:
  - Character designer and sheets
  - Chat and video conference
  - 3d map with webgl
  - chatbot
- AI enemies and chatbots for single player play

Most likely, I will proteanfit the existing khadga app for this.  In fact, I may wind up adding this to the multirepo in
the khadga project.

## Programming language consideration

I decided to write this in rust for a couple of reasons:

- I want to get better at rust
- Since it's rust, the shared library will be able to be used by C(++) applications
- I can also compile it down to webassembly for use in browsers
- It's blazing fast and it needs to be

It would be nice to do this in more popular languages to get more contributions, so I might wind up doing some pieces
in typescript (especially parts for tensorflow).