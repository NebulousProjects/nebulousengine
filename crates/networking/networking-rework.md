### Current Problems
1. Currently is direct P2P, no relay.  This means low security and no way to check if a player is cheating.
2. Lots of teleport events, which hog network resources and makes replay difficult as all that data needs to be stored.

### New plan
1. We need a relay server.  This server sholud track all events on the server.  It should also have the ability to "replay" a set of events to check them if a client flags a set of events.
2. The communication should be event based with timestamps.  This should allow the server and clients to "replay" events if something changes.
3. These events can also allow for non-random AI to also be played without control from any clients.
4. These events can also be allowed to manage non-random game states (like when the players should proceed to the game state from a selection state).

### Cool ideas
1. Create actions system with that can be given functions to run when they are applied that do not necessarily require a network to function (probably will need a new events system)
   1. The ability to have these actions retracted could be very useful for AI and anticheat
   2. Time stamps would be useful for later features and "replaying" the game world
   3. Could use a action user for applying and retracting actions (maybe they should be passed the last couple similar actions)
2. Create character controllers that can work off these actions (like set direction and speed actions, jump actions, etc)
   1. The controller should be able to place itself at any point back in time using those actions if a new event arrives or an action is retracted
3. Create "statemachines" that can have their clocks "turned back" if an action arrives that is a little bit pre-dated one of its decisions
   1. Will need to have a decision system that is based on the above eveactionsnts system

[ ] Implement actions module
[ ] Some way to flag actions for being possibly incorrect
[ ] A basic controller that has 
[ ] Network the actions