#!/bin/bash

# Download ancillary resources for processing

resources=(
    # Alexa top 1 million domains
    # https://s3.amazonaws.com/alexa-static/top-1m.csv.zip

    # Cisco Umbrella
    # http://s3-us-west-1.amazonaws.com/umbrella-static/top-1m.csv.zip

    # Majestic Million
    # http://downloads.majesticseo.com/majestic_million.csv

    # Tranco
    https://tranco-list.eu/top-1m.csv.zip

    # Public Suffix List
    https://publicsuffix.org/list/public_suffix_list.dat

    # GeoLite 2 ASN database
    https://geolite.maxmind.com/download/geoip/database/GeoLite2-ASN.tar.gz
);

dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )";
resource_dir="${dir}/../resources/";

cd $resource_dir;

for file in "${resources[@]}"; do
    wget $file -P $resource_dir 2>/dev/null || curl -O $file;
done

# Unpack everything
unzip *.zip;
tar --strip=1 -zxvf *.tar.gz;
