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

bbb = serial.Serial('/dev/ttyUSB0', baudrate=115200, timeout=.1)
time.sleep(3)

loopnum = 0

bbb.write(bytes("hello!", 'utf-8'))
print("done!")


while True:

    data = bbb.readlines()

    if loopnum % 5 == 0:
        bbb.write(bytes(str(data), 'utf-8'))
    else:
        bbb.write(bytes("hello world\n", 'utf-8'))      

    loopnum += 1

    time.sleep(1)
    