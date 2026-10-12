# `make gate` is the Definition of Done for this repository. CI runs it on
# every pull request; run it locally before you push.

CRATES := $(notdir $(wildcard crates/*))
PV_OUT := target/pv

.PHONY: gate fmt clippy test pv pv-validate pv-lint pv-ontology

gate: fmt clippy test pv

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

pv: pv-validate pv-lint pv-ontology

# Every contract is schema-valid.
pv-validate:
	@for f in contracts/*-v1.yaml; do pv validate -q $$f || exit 1; done

# Every crate has a contract and a binding, and pv lint passes with the
# strict test-binding gate: every falsification test the contract names is a
# real #[test]. The JSON is checked too, so a failing gate that pv does not
# arm still fails here.
pv-lint:
	@mkdir -p $(PV_OUT)
	@for c in $(CRATES); do \
	  test -f contracts/$$c-v1.yaml || { echo "$$c: no contracts/$$c-v1.yaml"; exit 1; }; \
	  test -f contracts/$$c/binding.yaml || { echo "$$c: no contracts/$$c/binding.yaml"; exit 1; }; \
	  pv lint contracts --binding contracts/$$c/binding.yaml --crate-dir crates/$$c \
	    --strict-test-binding --strict -f json > $(PV_OUT)/lint-$$c.json || { echo "$$c: pv lint failed"; exit 1; }; \
	  jq -e '[.gates[] | select(.verdict == "Fail")] | length == 0' $(PV_OUT)/lint-$$c.json > /dev/null \
	    || { echo "$$c: a pv lint gate failed, see $(PV_OUT)/lint-$$c.json"; exit 1; }; \
	  echo "$$c: pv lint ok"; \
	done

# The signature Σ exports to OWL and its TBox is consistent.
pv-ontology:
	@mkdir -p $(PV_OUT)
	pv ontology export --owl contracts/ontology.yaml > $(PV_OUT)/ontology.owl
	pv ontology tbox contracts/ontology.yaml > $(PV_OUT)/tbox.json
	jq -e '.consistent == true' $(PV_OUT)/tbox.json > /dev/null
