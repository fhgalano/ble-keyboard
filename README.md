# Overview 

This is an implementation of a bluetooth keyboard on an ESP32-C3 Devkit

# Code 

This was developed in conjunction with [keyboard](https://github.com/fhgalano/keyboard)

I'm using [esp32-nimble](https://github.com/taks/esp32-nimble) to handle the bluetooth conection. Most of the bluetooth connection code is taken from the examples. 

# Mechanical

I've been having fun with kicad scripting - I like the consistency it creates. It also helps with the tedious parts of routing a bunch of keyboard keys. Feel free to check it out [here](https://github.com/fhgalano/kicad-utilities), but I would describe the code as 'certified jank'

The PCB for the keyboard is reversible and uses cherry mx compatible hotswap sockets. It uses most of the gpio pins available on the devkit, but any remaining pins are made available through jst ph-k through hole connectors. It's also only two layers to make it cheaper to iterate through.

_**as of Feb 2025 I have only just placed the order for the first revision of the pcb, so I have no idea if it works. Use at your own risk. I'll update once I get it.**_

*Update: the board works but also has some large flaws - the biggest of which being that the devkit is positioned such that I need to remove several keys in order to program it. I'm lucky that I made this with hotswaps. I don't plan to update the board for a while, despite the flaws - I'll be implementing some new features (quick switching between bonded devices, power saving, using multiple devices as one wirelessly, layer indication leds, timers instead of blocking delays, and other stuff I notice while using it). Once those are done, then I'll look at redesigning the board*

I plan to make documentation eventually, since it's good practice. However, it isn't a huge priority since I am literally the only person aware that this project exists. Also, anyone looking to make a keyboard should probably be using something more feature rich like QMK, RMK, or Keyberon. Nevertheless, if documentation would help your project/learning then I'd be happy to add it in. 
