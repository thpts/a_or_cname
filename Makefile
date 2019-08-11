.PHONY: resources prepare collector collector-build analysis-build analysis

build: analysis-build collector-build

collector-build:
	docker build -t dnsobs-collector collector/

analysis-build:
	docker build -t dnsobs-analysis analysis/

prepare:
	bin/prepare_database.sh -d $(db)

analysis:
	docker run --rm -v ${PWD}/analysis:/analysis -v ${PWD}/data:/data dnsobs-analysis

resources:
	bin/fetch_resources.sh
