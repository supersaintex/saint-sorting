#!/bin/zsh

#TIMEOUT=300
#count=0
#PID=$$
#cmd="ps aux | grep ' target/debug/saint-sorting' | grep -v grep | wc -l"
#pn=$(eval $cmd)
#if [ $pn -eq 1 ]; then
#  echo "app already running; kill & run..."
#  pid=`ps aux | grep ' target/debug/saint-sorting' | grep -v grep | awk '{print $2}'`
#  kill -9 $pid
#fi
#(
#  while :
#  do
#    cmd2="ps $PID | wc -l"
#    pn2=$(eval $cmd2)
#    pn=$(eval $cmd)
#    if [ $pn2 -ne 2 ]; then # cargo run failed
#      exit 1
#    fi
#    if [ $pn -eq 1 ]; then
#      exit 0
#    fi
#    sleep 1
#  done
#)&
#echo "hello"
{
	cd thirtyfour_test
	java -jar *.jar --ext example.jar:dir standalone
}&
{
	./run_without_browser.zsh
	#./run_with_browser.zsh
}&
{
	cd thirtyfour_test
	sleep 1
	while :
	do
		cmd="ps aux | grep ' target/debug/saint-sorting' | grep -v grep | wc -l"
		pn=$(eval $cmd)
		cmd3="ps aux | grep ' selenium' | grep -v grep | wc -l"
		pn3=$(eval $cmd3)
		if [ $pn -eq 1 ] && [ $pn3 -eq 1 ]; then
			cargo run
			exit 0
		fi
		sleep 1
	done
}&
#cargo run &
#cd thirty_four
#cargo run
