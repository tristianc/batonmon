#!/usr/bin/python

# remote_listener.py
from evdev import InputDevice, categorize, ecodes, list_devices
import subprocess

target_name = "Sofabaton03B03 Consumer Control"  # e.g., "Bluetooth Remote"
devices = [InputDevice(path) for path in list_devices()]

for device in devices:
    if target_name in device.name:
        dev = InputDevice(device.path)
        break
else:
    raise Exception("Device not found")

for event in dev.read_loop():
    if event.type == ecodes.EV_KEY:
        key_event = categorize(event)
        if key_event.keystate == key_event.key_down:
            if key_event.keycode == 'KEY_SEARCH':  # Replace with your button
                subprocess.Popen(['./script.fish'])

