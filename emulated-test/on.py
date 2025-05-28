# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "pyserial",
# ]
# ///

import serial
import time
import string

# Open serial port at 115200 baud to match typical Fluctus settings
ser = serial.Serial('/dev/ttys005', 115200)

# Example packet from the documentation
test_packet = "FB3E00070100BEDD01000000000000006C00AA89109CFF00650000000000000000000E53000000|Grssi-65/Gsnr6\n"

i = 0
while True:
    ser.write(test_packet.encode('utf-8'))
    print(f"Sent packet #{i}")
    i += 1
    time.sleep(0.1)  # Send once per second
