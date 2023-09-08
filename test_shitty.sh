#!/bin/bash

echo "debug signal"
./matt_daemon
kill -1 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock

echo "debug signal"
./matt_daemon
kill -2 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -3 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -4 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -5 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -6 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -7 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -8 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -10 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -11 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -12 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -13 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -14 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -15 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -16 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -18 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -19 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -21 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -22 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -23 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -24 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -25 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -26 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -27 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -28 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -29 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -30 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
 
echo "debug signal"
./matt_daemon
kill -31 $(pgrep matt)
ps -A ; ls /var/lock/matt_daemon.lock
