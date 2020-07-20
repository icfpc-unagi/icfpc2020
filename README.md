# Team Unagi

Team Unagi's repository for ICFPC 2020

## Members

- Takuya Akiba
- Kentaro Imajo
- Hiroaki Iwami
- Yoichi Iwata
- Toshiki Kataoka
- Naohiro Takahashi


## Programming Language

We thoroughly used Rust for writing a galaxy interpreter, game bots, etc. We also used go and javascript for human interaction.


## Approaches

### Attacker

There were two major Defender algorithms employed by the top players.
- run away from Attacker
- take away Attacker's time by splitting up
These two tactics involve closing the distance quickly and attacking as quickly as possible.

For this reason, we have implemented a routine to move as quickly as possible to the objective with width priority search. This tactic increases attack power and recovery speed, and defeats the opponent in a fast attack.
However, this tactic will run out of energy quickly because of the heavy movement. So, when we were about to run out of Energy, we found a trajectory that would allow us to continue to drift through space without eruptions, thus conserving energy.

For enemy attacks, we implemented a combination of two algorithms: one that observes the enemy's movements and statistically predicts where they will move, and another that assumes that they will repeat the previous move as they move.
For artillery damage, we were able to discover the formula itself by analyzing the damage one square at a time. Using these, we selected the necessary artillery strikes at random.



### Defender

For each point p and each speed v, we precompute, by reversed BFS,  how many turns a ship would die starting from p with v without using any boost.
At the start of the game, we move the ship into an orbit that allows it to survive until the end of the game without using any boost.
Once in orbit, it splits, one moves to another orbit, and each one repeats the split recursively.
After about 15 turns, the ship will split into 96 ships.
Each ship continues to orbit, and when an enemy ship comes close, it will detonate.

