#!/usr/bin/env bash
dir="/home/max/Documents/Aurea/projects/placeable.com/trail/elastic-data/app/akka-quickstart-scala/raw-data/d_all_txt/data/daily/us/nyse etfs/"
#find "/home/max/Documents/Aurea/projects/placeable.com/trail/elastic-data/app/akka-quickstart-scala/raw-data/d_all_txt/data/daily/us/nyse etfs/" -maxdepth 1 -type f | \
#  ../combin/target/release/combin -p 3 | \
#  xargs -P12 -i -0 -d "\n" target/release/threads -f 2009 -t "/home/max/Documents/Aurea/projects/placeable.com/trail/rust/results/nyse_etfs/2009" '{}'
#find "/home/max/Documents/Aurea/projects/placeable.com/trail/elastic-data/app/akka-quickstart-scala/raw-data/d_all_txt/data/daily/us/nyse etfs/" -maxdepth 1 -type f | \
#  ../combin/target/release/combin -p 3 | \
#  xargs -P12 -i -0 -d "\n" target/release/threads -f 2009 -t "/tmp/results" '{}'
year=2005
echo "start from $year"
dir="/tmp/1"
target_dir="/home/max/Documents/Aurea/projects/placeable.com/trail/rust/results/current"
#rm -r "${target_dir}/*"

#files filtered by year
fd . "$dir" | \
# |\
  ../combin/target/release/combin -p 2 |\
 #   tail -n1 |\
    xargs -P12 -i -0 -d "\n" target/release/stats -b '{}' | tee /tmp/res/res1.txt
#target/release/stats -b "/tmp/1/spy.us.txt" "/tmp/1/^rts.txt"
#all files
#fd . "$dir"| \
#  ../combin/target/release/combin -p 9 |\
#  xargs -P12 -i -0 -d "\n" target/release/threads -v 2 -t "${target_dir}" '{}'
