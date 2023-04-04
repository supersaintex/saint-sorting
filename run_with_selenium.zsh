#!/bin/zsh
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
		if [ $pn -eq 1 ]; then
		  	cargo run
			exit 0
		fi
		  	sleep 1
	done
}&
