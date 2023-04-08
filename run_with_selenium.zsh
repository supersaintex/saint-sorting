#!/bin/zsh
{
	cd thirtyfour_test
	java -jar *.jar --ext example.jar:dir standalone
}&
{
	./run_with_browser.zsh
}&
{
	cd thirtyfour_test
	sleep 1
	while :
	do
		cmd="ps aux | grep ' target/debug/saint-sorting' | grep -v grep | wc -l"
		pn=$(eval $cmd)
		cmd2="ps aux | grep ' selenium' | grep -v grep | wc -l"
		pn2=$(eval $cmd2)
		if [ $pn -eq 1 ] && [ $pn2 -eq 1 ]; then
			cargo run
			exit 0
		fi
		sleep 1
	done
}&
