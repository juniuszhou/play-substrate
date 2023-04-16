.PHONY: test_pallet
test_pallet:
	cargo test --release --package play-balances -- --nocapture tests::pallet_info


