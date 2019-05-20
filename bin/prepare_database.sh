#!/bin/bash

# Create the database and populate it with domain listing

# The database file MUST be created in the data/ subdirectory.

usage() {
    echo "Usage: $0 -d <SQLite Database>" 1>&2;
    exit 1;
}

while getopts ":d:" o; do
    case "${o}" in
        d)
            DB=${OPTARG}
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

csv_file="/resources/top-1m.csv";
ps_db="/resources/public_suffix_list.dat";
sqlite_db="/data/${DB}"
docker_args="-v ${data_dir}:/data -v ${resources_dir}:/resources"

docker run ${docker_args} ${container_name} database_setup --sqlite-db ${sqlite_db};
docker run ${docker_args} ${container_name} domain_load --csv ${csv_file} \
                                                        --public-suffix-db ${ps_db} \
                                                        --sqlite-db ${sqlite_db};
