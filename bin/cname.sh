#!/bin/bash

# Fetch and process the Alexa 1m domain list.

wget https://s3.amazonaws.com/alexa-static/top-1m.csv.zip;
unzip top-1m.csv.zip;

<top-1m.csv awk -F, '{print $2}' |\
  xargs -I{} bash cname_dig.sh {} |\
  tee top1m_cnames.csv;

