#!/bin/zsh

TIMEOUT=300
count=0
PID=$$
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
(
	cd thirtyfour_test
	java -jar *.jar --ext example.jar:dir standalone
)&
{
	sleep 1
	while :
	do
		cmd3="ps aux | grep ' selenium' | grep -v grep | wc -l"
		pn3=$(eval $cmd3)
		if [ $pn3 -eq 1 ]; then
			./run_without_browser.zsh
			exit 0
		fi
		sleep 1
	done
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
# kill application and selenium when the test ends.
sleep 3
while :
do
	cmd="ps aux | grep ' target/debug/saint-sorting' | grep -v grep | wc -l"
	pn=$(eval $cmd)
	cmd2="ps aux | grep ' target/debug/thirtyfour_test' | grep -v grep | wc -l"
	pn2=$(eval $cmd2)
	if [ $pn -eq 1 ] && [ $pn2 -eq 0 ]; then
		echo "The test ends, and kill the application and selenium server..."
		sleep 3
		pid=`ps aux | grep ' target/debug/saint-sorting' | grep -v grep | awk '{print $2}'`
		kill -9 $pid
		pid2=`ps aux | grep ' selenium' | grep -v grep | awk '{print $2}'`
		kill -9 $pid2
		exit 0
	fi
	sleep 1
done
