#!/bin/bash

# Fetch and process the Alexa 1m domain list.

dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )";
resource_dir="${dir}/../resources/ietf-dnsop-2018-11-21/";

mkdir -p $resource_dir;
cd $resource_dir && wget https://s3.amazonaws.com/alexa-static/top-1m.csv.zip;
unzip top-1m.csv.zip;

<top-1m.csv awk -F, '{print $2}' |\
  xargs -I{} bash cname_dig.sh {} |\
  tee top1m_cnames.csv;

