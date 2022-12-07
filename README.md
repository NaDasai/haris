Smart contracts, at least on Ethereum, are static code. This means there’s no way for a smart contract to do anything without a transaction being first initiated by an Externally Owned Account (EOA).

This is the reason Keepers, Liquidators, Arbitrageurs, etc. exist. They are necessary to execute the operations that keep dApps operating properly.

MakerDAO, for example, has a set of Keepers that monitor the liquidation price of collateralized debt positions (CDPs). If the collateralization rate falls below a specific number, these Keepers trigger the liquidation of a vault — ensuring the position is fully collateralized.

Taking notes on this:

---

Sentinel As A Service.

A bot that aggregates super token data and handles a configurable array of function calls.

Functionally, it's similar to a keeper, but is more narrowly focused on superfluid related operations.

Ideal stuff:
- agg super token data
- liquidation capable
- courtesy closure capable
- arbitrary function capable
- meta transaction capable
- high performance (Rust :thonk:)