A crate to parse information contained in the [dps
calc](https://docs.google.com/spreadsheets/d/1wBXIlvAmqoQpu5u9XBfD4B0PW7D8owyO_CnRDiTHBKQ/).
Right now we're interested in the items. Later we'll also want the NPC data.

https://docs.google.com/spreadsheets/d/1wBXIlvAmqoQpu5u9XBfD4B0PW7D8owyO_CnRDiTHBKQ/



savant workflow

starts
- read in item database
- read in npc database
- read in any other needed datebase
- parse those into the format that will be used by our program
- build scenario using relevant inputs (stats, equipment, npc)
  - from a cli this might look like a config being passed on a command line
  - this would build scenario and then simulate it a certain number of times
- make calls against that scenario to get hits (accuracy, damage rng)



future:
do we want a client server model with a running server that contains all the
various info and clients that call against it? probably more complexity than we
need

