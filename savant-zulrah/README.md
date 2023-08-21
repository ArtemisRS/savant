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
git clone https://github.com/ArtemisRS/savant && cd savant/savant-zulrah
cargo run -c <config>
```

Savants loads options from a toml formatted config file.
