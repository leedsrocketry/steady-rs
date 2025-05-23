# Steady-rs

Steady-rs is a rust library for interfacing with the [Silicdyne Fluctus flight computer](https://silicdyne.net/fluctus/) via the [Steady ground station](https://silicdyne.net/steadyblue/).

All it does is provide tools for reading data coming in via the Steady ground station's serial connection. There is no system for sending and receiving that data in the library. This may change.

This aims to fully implement the interface as specified in the [inital documentation](http://silicdyne.net//resources/docs/fluctus_sgs_interface_protocol_1_7b.pdf) provided.

Shoutout to Ulysse from Silicdyne, who created documentation specifically for this very specific usecase we had. We are using this to feed data into our custom unified ground station.