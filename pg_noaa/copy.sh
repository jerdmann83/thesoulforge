#!/bin/bash
# super yolo copy/import script
# no error handling, just eat errors and keep going
for f in *.csv; do 
    fname=$(readlink -f $f)
    echo "import $fname"
    psql -d noaa -f noaa-copy.sql -v filename="$fname"
done
