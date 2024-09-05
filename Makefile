.DEFAULT_GOAL := help
SHELL:=/bin/bash

## Build static binary at target/debug/asfd, requires docker.
## Choose profile with variable PROFILE (dev, release,test,bench). Default = dev
linux-static:
	docker build -t asfd-build .
	docker run -v $$PWD:/asfd -w /asfd -u "$$(id -u):$$(id -g)" -it --rm asfd-build ash -c "OPENSSL_STATIC=1  OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include cargo build $${PROFILE:+--$${PROFILE}}"

## RELEASE step 1: Start our workflow generating artifacts.
gh-build-binaries:
	@echo "starting workflow"
	gh workflow run .github/workflows/ci.yml
	@echo "take note of the just started run"
	gh run list
	@echo "-------------------------------------------------------------------------"
	@echo "now wait for the run to be finished, eg with make gh-wait-run RUN_ID=XXXX"
	@echo "-------------------------------------------------------------------------"

## RELEASE step 2: Wait for run RUN_ID to complete
gh-wait-run:
	[[ -n "$(RUN_ID)" ]] || { echo -2 "RUN_ID is required" ; exit 1; }
	while [[ $$(gh run list --json databaseId,status -q '.[] | select (.databaseId==$(RUN_ID)).status') != "completed" ]]; do  \
		echo "Waiting for run to completed"; \
		sleep 10; \
	done
	@echo "---------------------------------------------------------------------------------"
	@echo "now download the artifacts, eg with make gh-download-artifacts RUN_ID=$(RUN_ID)"
	@echo "---------------------------------------------------------------------------------"

## RELEASE step 3: Download all artifacts of run RUN_ID
gh-download-artifacts:
	[[ -n "$(RUN_ID)" ]] || { echo -2 "RUN_ID is required" ; exit 1; }
	gh run download $(RUN_ID)
	@echo "-------------------------------------------------------------------------"
	@echo "now you can prepare the release locally, eg with make gh-prepare-release"
	@echo "-------------------------------------------------------------------------"

## RELEASE step 4: Create a release/ directory and generate files of a Github release in it.
gh-prepare-release:
	mkdir release; \
  for dir in asfd-*; do \
		cp LICENSE $$dir/; \
		if [[ ! $$dir =~ windows ]]; then chmod +x $$dir/asfd; fi; \
		tar zcvf release/$$dir.tar.gz $$dir; \
	done; \
	(cd release; sha256sum * > checksums.txt;)
	@echo "-------------------------------------------------------------------------"
	@echo "The release artifacts are available under release/"
	@echo "-------------------------------------------------------------------------"

help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)"
	@echo
	@sed -n -e "/^## / { \
		h; \
		s/.*//; \
		:doc" \
		-e "H; \
		n; \
		s/^## //; \
		t doc" \
		-e "s/:.*//; \
		G; \
		s/\\n## /---/; \
		s/\\n/ /g; \
		p; \
	}" ${MAKEFILE_LIST} \
	| LC_ALL='C' sort --ignore-case \
	| awk -F '---' \
		-v ncol=$$(tput cols) \
		-v indent=19 \
		-v col_on="$$(tput setaf 6)" \
		-v col_off="$$(tput sgr0)" \
	'{ \
		printf "%s%*s%s ", col_on, -indent, $$1, col_off; \
		n = split($$2, words, " "); \
		line_length = ncol - indent; \
		for (i = 1; i <= n; i++) { \
			line_length -= length(words[i]) + 1; \
			if (line_length <= 0) { \
				line_length = ncol - indent - length(words[i]) - 1; \
				printf "\n%*s ", -indent, " "; \
			} \
			printf "%s ", words[i]; \
		} \
		printf "\n"; \
	}' \
	| more $(shell test $(shell uname) == Darwin && echo '--no-init --raw-control-chars')
