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

### Zulrah simulation
Simulates attacks against Zulrah using specific setups for each phase.
Calculates the number of attacks that can be completed for each phase based
on the selected rotation.

Features:
 - Supports separate setups for each phase (Serpentine, Crimson, Tanzanite)
 - Supports all four rotations
 - Reports a tick accurate Time To Kill (ttk) for each rotation
 - Simple statistics (average, percentile) for each rotation and overall

Future features:
 - Pillar stalling - This requires a large rework to fully support
 - Mixed setups on a given phase (last hits, tick maximizing, etc)
 - Spec weapons (which will depend on above)
 - Bolt specs (likely just ruby + diamond)
 - Specifying the weapon/setup for every attack on a given rotation/phase

Things Unlikely to be supported:
 - Veng
 - Thralls
 - Delay from eating

### Usage
Clone and run locally. This will require Rust to be installed.
```bash
git clone https://github.com/ArtemisRS/savant && cd savant
cargo run -c <config>
```

Savants loads options from a toml formatted config file.
