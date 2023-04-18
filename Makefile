SRC_DIR = ./contract
TS_OUTPUT_DIR = ts

generate_ts:
	@for file in $(wildcard $(SRC_DIR)/*); do \
		file_name=$$(basename $$file); \
        file_name_without_extension=$${file_name%%.*}; \
		 echo "Processing file: $$file_name_without_extension"; \
         typeshare $(SRC_DIR) --lang=typescript --output-file=$(TS_OUTPUT_DIR)/output.ts; \
   done

install_rust:
	@command -v rustc >/dev/null 2>&1 || { \
		echo >&2 "Rust n'est pas installé. Installation en cours..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
		source $$HOME/.cargo/env; \
	}

install_typeshare_cli:
	@command -v typeshare-cli >/dev/null 2>&1 || { \
		echo >&2 "typeshare-cli n'est pas installé. Installation en cours..."; \
		cargo install typeshare-cli; \
	}