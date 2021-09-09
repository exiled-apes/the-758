
exiled-holders.log: exiled-token-addresses.log
	@cargo run -- list-holders < exiled-token-addresses.log > exiled-holders.log

exiled-metadata.log: exiled-token-addresses.log
	@cargo run -- list-metadata < exiled-token-addresses.log > exiled-metadata.log

exiled-token-addresses.log:
	@cargo run -- list-exiles | sponge exiled-token-addresses.log