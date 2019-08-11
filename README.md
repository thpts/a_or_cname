# A or CNAME?

This repository contains analysis originally performed in November 2018 after a
discussion in the IETF dnsop WG mailing list on the subject of website usage of
CNAME records and A records in their zone's apex.

## Structure

* `analysis` - Notebooks and other data and prose analysing the data
* `bin` - Collection of scripts to fetch data and perform ancillary tasks
* `collector` - Executables for collecting DNS data
* `data` - Output directory for data collected
* `resources` - Holding directory for supplementary data sets

## Building and Running

You will need Docker and bash present in order to generate the dataset yourself
and either `wget` or `curl` to download resource data sets such as the ASN
database, Alexa 1m CSV, etc.

```bash
    # Compile the collector and analysis containers
    make build

    # Initialise the dataset, set 'db' to your preferred filename in data/
    make db=test_run.sqlite prepare

    # Fetch additional data sets
    make resources

    # Generate the reporting
    make analysis
```

## Copyright, Licensing, and Attribution

This product includes GeoLite2 data created by MaxMind, available from
[https://www.maxmind.com](https://www.maxmind.com).

We use the Tranco list, available at
[https://tranco-list.eu/](https://tranco-list.eu/).

The Public Suffix List &copy; Mozilla Foundation and licensed under the [Mozilla
Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

Source code within this repository is licensed under the [Apache
2](http://www.apache.org/licenses/LICENSE-2.0) software license.

Prose and non-software is licensed under the [Creative Commons Attribution 2.0
Generic](https://creativecommons.org/licenses/by/2.0/) license. Referring to
this code repository on Github along with author's names is sufficient in order
to be compliant with attribution.
