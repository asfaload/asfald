.DEFAULT_GOAL := help
SHELL:=/bin/bash

## Build static binary at target/debug/asfald, requires docker.
## Choose profile with variable PROFILE (dev, release,test,bench). Default = dev
linux-static:
	docker build -t asfald-build .
	docker run -v $$PWD:/asfald -w /asfald -u "$$(id -u):$$(id -g)" -it --rm asfald-build ash -c "OPENSSL_STATIC=1  OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include cargo build $${PROFILE:+--$${PROFILE}}"

## Perform validations of the code and compilation (warnings are errors).
check:
	cargo rustc -- -D warnings

# name of the tmux session running the http server started for the tests
TMUX-SESSION := asfaload-test-http-server
# shell command to detect if the tmux session exists
TMUX-SESSION-EXISTS := tmux has-session -t $(TMUX-SESSION) &> /dev/null
TMUX-SESSION-EXISTS_2 := tmux has-session -t $(TMUX-SESSION)_2 &> /dev/null
CURRENT_DIR := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
start-test-server:
	$(TMUX-SESSION-EXISTS) || tmux new-session -s "$(TMUX-SESSION)" -c $(CURRENT_DIR)/tests/data/server1 -d python -m http.server 9988
	$(TMUX-SESSION-EXISTS_2) || tmux new-session -s "$(TMUX-SESSION)_2" -c $(CURRENT_DIR)/tests/data/server2 -d python -m http.server 9989

stop-test-server:
	$(TMUX-SESSION-EXISTS) && tmux kill-session -t "$(TMUX-SESSION)" || true
	$(TMUX-SESSION-EXISTS_2) && tmux kill-session -t "$(TMUX-SESSION)_2" || true

## Starts a local http server in tmux and run the tests, before stopping the server.
test: start-test-server
	cargo test
	$(MAKE) stop-test-server


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
