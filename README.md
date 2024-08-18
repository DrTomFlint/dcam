Using Rust make a fast modern security camera application.
Choose Rocket as the web framework / server.
Raspberry Pi Zero W2 and 4b will host cameras with pan and tilt servos.
Possibly some weather instruments, compass, tilt sensor, air quality?
The libcamera-rs crate will be used for acquiring images and streams.
A Debian Linux Hub will aggregate the data and present a unified web page.
The Hub should poll the remote RPi which should provide the requested 

dcampi is code for the Raspberry Pi cameras.
dcamhub is code for the Linux machine.

On the development Linux host set up for cross-compiling to PiZeroW2 target:
sudo apt install gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu

In dcampi there is a "deploy" script that builds for the Pi, rsync files, and run using ssh.
That assumes a pre-shared ssh key so you don't need passwords.

On the development machine:
ssh-copy-id -i .ssh/id_rsa.pub tflint@cam4.local
