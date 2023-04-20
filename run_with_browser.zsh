#!/bin/zsh

TIMEOUT=300
count=0
PID=$$
cmd="ps aux | grep ' target/debug/saint-sorting' | grep -v grep | wc -l"
pn=$(eval $cmd)
if [ $pn -eq 1 ]; then
  echo "app already running; kill & run..."
  pid=`ps aux | grep ' target/debug/saint-sorting' | grep -v grep | awk '{print $2}'`
  kill -9 $pid
fi
(
  while :
  do
    cmd2="ps $PID | wc -l"
    pn2=$(eval $cmd2)
		pn=$(eval $cmd)
    if [ $pn2 -ne 2 ]; then # cargo run failed
      exit 1
    fi
    if [ $pn -eq 1 ]; then
      google-chrome https://127.0.0.1:8080/app/top
      exit 0
    fi
    sleep 1
  done
)&
cargo run
