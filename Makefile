
exiled-metadata.log: exiled-token-addresses.log
	@cargo run -- list-metadata < exiled-token-addresses.log > exiled-metadata.logx

exiled-token-addresses.log:
	@cargo run -- list-exiles | sponge exiled-token-addresses.log