# Savant
Savant is a combat simulator for Old School RuneScape. The goal is to provide
accurate simulations of boss fights that are too complex for numerical
analysis with existing tools.

### Status
Savant is highly incomplete. Currently, there is only a simple simulator for
Zulrah, which is detailed below. There is unfinished (non public work) to
extend Savant to simulate other bosses including Olm, Cerberus, and Vorkath.

Right now, Savant does not do DPS calculations, but relies on precalculated
max hits and accuracy. It would be nice to integrate a DPS calculator for
ease of use, but supporting enough items to be useful would be too time
intensive.

### Organization
bulk of the existing code is in the subcrate savant-zulrah. See the README there
for directions on how to run. The other crates contain work to introduce a
general purpose DPS calculator.
