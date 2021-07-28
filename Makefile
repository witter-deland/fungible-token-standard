.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test_method_supported_interface
.SILENT: test_method_supported_interface
test_method_supported_interface: upgrade 
	dfx canister call dft supportedInterface '("supportedInterface:(text)->(bool) query)")' \
		| grep '(true)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx