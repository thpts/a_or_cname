#!/bin/bash

# For each record, dig 'www' and NS of the apex

export DOMAIN=$1
export A_OR_CNAME=$(dig +noall +answer www.${DOMAIN} 2>/dev/null | head -1 | awk '{$1=$1}1' ORS="" OFS=",");
export NS=$(dig +short NS ${DOMAIN} 2>/dev/null | awk -v ORS="," '2'| sed s'/,$//');

echo "${DOMAIN},${A_OR_CNAME},${NS}" |\
  # Only print select fields:
  # domain, www, class, record type, answer, and first four name servers
  awk -F, '{OFS=","};{print $1,$2,$4,$5,$6,$7,$8,$9,$10}';