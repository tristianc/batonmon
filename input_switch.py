#!/usr/bin/python

import re
from evdev import InputDevice, categorize, ecodes, list_devices
import subprocess
import time

target_name = "Sofabaton03B03 Consumer Control"  # e.g., "Bluetooth Remote"

class UnknownOutput(Exception):
    pass

def toggle_input():
    display = 2
    result = subprocess.run(["ddcutil", "-d", "2", "getvcp", "60"], capture_output=True)
    m = re.search(r'\(sl=(?P<id>.*)\)',str(result.stdout))
    try:
        output = ""
        current = m.group('id')
        if current == "0x19":
            output = "0x0f"
        elif current == "0x0f":
            output = "0x19"
        else:
            raise UnknownOutput
        print(f"Setting to output {output}") 
        proc = subprocess.run(["ddcutil", "-d", str(display), "setvcp", "60", output], text=True)
        proc.check_returncode()
    except (IndexError, UnknownOutput) as err:
        print("Unknown display output")
        return
    except CalledProcessError as err:
        print(f"Failed to invoke ddcutil: {proc.stderr}")
        return

while True:
    devices = [InputDevice(path) for path in list_devices()]
    dev = None
    for device in devices:
        if target_name in device.name:
            dev = InputDevice(device.path)
            break

    if not dev:
        print("Device not found")
        time.sleep(2)
        continue

    try:
        for event in dev.read_loop():
            if event.type == ecodes.EV_KEY:
                key_event = categorize(event)
                if key_event.keystate == key_event.key_down:
                    if key_event.keycode == 'KEY_SEARCH':  # Replace with your button
                        toggle_input()
    except OSError as err:
        print("Device lost")
