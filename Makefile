## print the help message.
# Parses this Makefile and prints targets that are preceded by "##" comments.
help:
	@echo "" >&2
	@echo "Available targets: " >&2
	@echo "" >&2
	@awk -F : '\
			BEGIN { in_doc = 0; } \
			/^##/ && in_doc == 0 { \
				in_doc = 1; \
				doc_first_line = $$0; \
				sub(/^## */, "", doc_first_line); \
			} \
			$$0 !~ /^#/ && in_doc == 1 { \
				in_doc = 0; \
				if (NF <= 1) { \
					next; \
				} \
				printf "  %-15s %s\n", $$1, doc_first_line; \
			} \
			' <"$(abspath $(lastword $(MAKEFILE_LIST)))" \
		| sort >&2
	@echo "" >&2


## provisions a working local development environment
dev.up:
	$(MAKE) -C devenv run

## run the roxy proxy listening on the default port
run:
	cargo run