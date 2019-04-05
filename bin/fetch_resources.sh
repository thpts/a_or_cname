#!/bin/bash

# Download ancillary resources for processing

resources=(
    # Alexa top 1 million domains
    https://s3.amazonaws.com/alexa-static/top-1m.csv.zip
    # Public Suffix List
    https://publicsuffix.org/list/public_suffix_list.dat
    # GeoLite 2 ASN database
    https://geolite.maxmind.com/download/geoip/database/GeoLite2-ASN.tar.gz
);

dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )";
resource_dir="${dir}/../resources/";

for file in "${resources[@]}"; do
    wget $file -P $resource_dir 2>/dev/null || cd $resource_dir && curl -O $file;
done

# Unpack everything
cd $resource_dir;
unzip *.zip;
tar -zxvf *.tar.gz;