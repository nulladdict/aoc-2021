.PHONY: dbg-%
dbg-%: day = $(subst dbg,day,$@)
dbg-%:
	cargo run -p $(day) < "pkgs/$(day)/in"

.PHONY: run-%
run-%: day = $(subst run,day,$@)
run-%:
	cargo run -p $(day) --quiet --release < "pkgs/$(day)/in"
