# Steady-rs

Steady-rs is a rust library for interfacing with the [Silicdyne Fluctus flight computer](https://silicdyne.net/fluctus/) via the [Steady ground station](https://silicdyne.net/steadyblue/).

Provides functions for:
- Sending commands to the Steady ground station
- Receiving packets from the Fluctus via the Steady ground station over a serial connection.

This aims to fully implement the interface as specified in the [inital documentation](http://silicdyne.net//resources/docs/fluctus_sgs_interface_protocol_1_7b.pdf) provided.

Shoutout to Ulysse from Silicdyne, who created documentation specifically for this very specific usecase we had.

We are using this to feed live flight data into our custom unified ground station solution.

There is relevant example code in emulated-test/src/main.rs.