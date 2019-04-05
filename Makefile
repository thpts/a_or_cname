.PHONY: resources collector analysis-build analysis

analysis-build:
	docker build -t dnsobs-analysis analysis/

analysis:
	docker run --rm -v ${PWD}/analysis:/analysis -v ${PWD}/data:/data dnsobs-analysis

resources:
	bin/fetch_resources.sh
