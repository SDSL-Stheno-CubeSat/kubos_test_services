# This code runs on a laptop/PC, which is connected through a UART USB to
# the beagleboard, which is running the uart_test rust project, every 5
# "hello world"s that are sent to the beagleboard, a "hello back" should be
# sent to the laptop/PC, which will send it back the the beagleboard, which
# will display it as an output. The output should look something like -

# hello world
# hello world
# hello world
# hello world
# [b'hello back]
# hello world
# ...

import serial
import time
import random

bbb = serial.Serial('/dev/ttyUSB0', baudrate=115200, timeout=.1)
time.sleep(3)

print('Connected to board!')

timer = time.time()

while True:

    data = bbb.readline()

    if data:
        print(data)

    if data.decode("utf-8") == "led":
        time.sleep(1)
        bbb.write(bytes("led complete - python", "utf-8"))
