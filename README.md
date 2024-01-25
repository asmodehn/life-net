# life-net
An attempt to make Conway's game of life multiplayer, somehow...

# Game of life

Classical Game of life has just a few rules, with cells (pixels here) that have only two states.

In order to make it multiplayer, we want to complexify this, yet keep the setup as simple as possible.

Ideas:

- dead cells corpse stay around and build sediments, which actually build terrain.
- => 3 states : without terrain, terrain (ie. dead cell), alive cell.
- for balance, terrain must disappear over time...
- cells can belong to various "tribes", for instance Red, Green and Blue for start.
- Reproduction picks the majority color from the neighbours, at random if no majority.

Then a player can choose a color, and can interact at any time with the game, by placing live cells of their color.
There might be some limitation to that ability however...

With these rules, we should have a game where multiple player can compete for domination of their color over the available terrain,
but also expand to new areas if needed...

# Engine

This game is also a test bed for a new kind of engine...

## Step 1

Since we will have multiplayer features, we have multiple side-effects rich interaction that may take significant computetime:
- simulation (pure cpu-memory only)
- rendering (to the screen)
- broadcasting (to the other players)
- maybe more depending on the kind of protocol is established between players.

The possible refresh rate accessible in these different cases varies a lot, and we might have a need for multiple nested loops in code,
or at least one loop with activable subsystems depending on time passed, to emulate having multiple nested loops...

## Step 2 

Once our game is ready to be played by multiple people we need to setup network communication. Two ideas:
- shared game events (eventually consistent, non-competitive since no referee, potentially slow/unstable )
- transient state update (non-consistent -> cosmetic only, non-competitive since no referee, potentially fast)

Depending on the choices made, the game design will need to be adapted to the capabilities of the network engine.


# Roadmap
- [X] traditional game of life
- [ ] multi-loop engine
- [ ] game of life with terrain
- [ ] game of life with multiple species
- [ ] user UI.
- [ ] MORE : towards multiplayer...