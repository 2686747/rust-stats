#!/usr/bin/env bash

target_dir="/tmp/res/"
cd "$target_dir" || exit 1
cat res1.txt| rg "clear:0"| rg  "spy"| awk -F ':' '{print $5"->"$6"\t"$0}' | sort -g | column -t | head -n 200


#how copy top of files
#rg "winner|loser" | awk -F '\t' '{print $6 "\t\t" $0}' | sort -gr | head -n10 | column -t | tee /tmp/top.txt
#cat /tmp/top.txt | awk '{print $2 }' | column -t|sed 's/:.*//;s/-/|/g'  |\
# xargs -I "?" -0 -d "\n" fd "?" "/home/max/Documents/Aurea/projects/placeable.com/trail/elastic-data/app/akka-quickstart-scala/raw-data/d_all_txt/data/daily/us/nyse etfs" -x cp "{}" /tmp/t
