# Overview 

This is an implementation of a bluetooth keyboard on an ESP32-C3 Devkit

# Code 

This was developed in conjunction with [keyboard](https://github.com/fhgalano/keyboard)

I'm using [esp32-nimble](https://github.com/taks/esp32-nimble) to handle the bluetooth conection. Most of the bluetooth connection code is taken from the examples. 

# Mechanical

I've been having fun with kicad scripting - I like the consistency it creates. It also helps with the tedious parts of routing a bunch of keyboard keys. Feel free to check it out [here](https://github.com/fhgalano/kicad-utilities), but I would describe the code as 'certified jank'

The PCB for the keyboard is reversible and uses cherry mx compatible hotswap sockets. It uses most of the gpio pins available on the devkit, but any remaining pins are made available through jst ph-k through hole connectors. It's also only two layers to make it cheaper to iterate through.

_**as of Feb 2025 I have only just placed the order for the first revision of the pcb, so I have no idea if it works. Use at your own risk. I'll update once I get it.**_
