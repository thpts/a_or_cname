#!/bin/bash

# Query the given resolver

usage() {
    echo "Usage: $0 -d <SQLite Database> -r <DNS Resolver>" 1>&2;
    exit 1;
}

while getopts ":d:r:" o; do
    case "${o}" in
        d)
            DB=${OPTARG}
            ;;
        r)
            DNS=${OPTARG}
            ;;
        *)
            usage
            ;;
    esac
done


container_name="dnsobs-collector";
dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )";
data_dir="${dir}/../data/";
resources_dir="${dir}/../resources/";
sqlite_db="/data/${DB}"
asn_db="/resources/GeoLite2-ASN.mmdb"
docker_args="-v ${data_dir}:/data -v ${resources_dir}:/resources"

echo "$(date -u +%FT%TZ): Querying against ${DB}...";

docker run ${docker_args} ${container_name} domain_query --asn-db ${asn_db} \
                                                         --resolver ${DNS} \
                                                         --sqlite-db ${sqlite_db}
