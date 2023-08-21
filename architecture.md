# Simulator design

As an API, we want to be able to generally build a simulation with player stats,
gear, setup(s), NPC, and any NPC properties. This lends itself to some kind of
builder layout, eg `sim.add_stats().add_npc().add_setup()` etc.

Zulrah specifically is a tricky case because we need a way to specify multiple
setups and strategies. I had assumed that I could base case with a "smart
enough" case, but that really won't work for pillar stalling.

I think we're looking at a general event loop that holds a "time passed" timer.
On every tick, the player and NPC will both be given a chance to attack if their
attack timer is up. 

```rust
while (npc.health > 0) {
while (npc.phase_timer > 0 ||  npc.remaining_attacks > 0) {
    if stalling {
        if player.off_cd() {
            player.attack(npc);
            if npc.off_cd() {
                npc.attack(player);
            }
        }
    } else { //not stlaling
        if npc.off_cd() {
            npc.attack(player)
        }
    }
}
}
```



Zulrah timings by phase (and number of zulrah attacks):

South green

            //cannot be stalled
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 20,

            //cannot be stalled
            form: Form::Magma,
            loc: Location::Middle,
            time: 14 // is this actually 2 8t attacks? how does magma timing work?

            //can be stalled
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 11, //4x 3t attacks

            //can stall the start
            form: Form::Serpentine,
            loc: Location::South,
            time: 33, //start with 5 3t attacks that can be stalled

            //cannot be stalled
            form: Form::Magma,
            loc: Location::Middle,
            time: 16, //suspicious that this is meant to be 2 8t attacks (15t available to attack)

            //can be stalled
            form: Form::Tanzanite,
            loc: Location::West,
            time: 14, //5x 3t attacks

            //cannot be stalled
            form: Form::Serpentine,
            loc: Location::South,
            time: 22, 

            //can be stalled
            form: Form::Tanzanite,
            loc: Location::South,
            time: 30, //suspicious of this timing

        //jad
            form: Form::Serpentine,
            loc: Location::West,
            time: 42,

            form: Form::Magma,
            loc: Location::Middle,
            time: 15,

            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
