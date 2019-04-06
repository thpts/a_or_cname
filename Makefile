.PHONY: resources collector collector-build analysis-build analysis

build: analysis-build collector-build

collector-build:
	cd collector && cargo build

analysis-build:
	docker build -t dnsobs-analysis analysis/

analysis:
	docker run --rm -v ${PWD}/analysis:/analysis -v ${PWD}/data:/data dnsobs-analysis

resources:
	bin/fetch_resources.sh
