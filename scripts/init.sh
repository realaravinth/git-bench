#!/bin/bash

readonly BASE_DIR=/tmp/git-bench.batsense.net

readonly EX_SOFTWARE=70
set -Euo pipefail


preflight(){
	for cmd in git
	do
		if ! command -v $cmd &> /dev/null
		then
			echo "$cmd COMMAND NOT FOUOND"
			exit $EX_SOFTWARE
		fi
	done
}


init() {
	rm -rf $BASE_DIR
	mkdir $BASE_DIR
	git init $BASE_DIR > /dev/null
	echo "Repository initialized at $BASE_DIR"
}

write_text() {
	text=$(cat /dev/urandom | env LC_ALL=C tr -dc 'a-zA-Z0-9' | fold -w 84 | head -n 100)
	echo $text > $1
}

populate() {
	echo "Generating data"
	for i in {1..10}
	do
		cd $BASE_DIR
		mkdir $i
		cd $i
		for j in {1..10}
		do
			write_text $j
			git add --all > /dev/null
			git commit -m "added file $i/$j" > /dev/null
		done
	done
}

preflight
init
populate
