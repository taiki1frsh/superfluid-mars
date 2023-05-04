# Superfluid Mars

## Concept

The goal is to **enable deposited OSMO **in Mars **to be** staked.****

Casually describing, `superfluid mars` can be achieved by the following features:

1. Tokenize bonds to know the amount of Osmosis being lent and to improve composability.

- Issuance of mOSMO (CW20)

2. (Low priority) Enable collateral management based on the presence or absence of bonds. For all transactions involving claims, use bond tokens.

- Locking of claim tokens when borrowing
- Burning of bond tokens when redeeming
- Burning (moving) of bond tokens when liquidating

3. Lock bond tokens in the Superfluid staking Contract, manage the rental usage rate of Osmo, and return the amount to Superfluid staking up to a certain proportion while not in use.
- Calculate the usage rate of Osmo in the Contract
- Calculate the maximum staking amount based on Params
- Hit a msg to update the staking amount
- Enable redemption after 14 days by unlocking

4. A Superfluid staking module for generic use cases of bond tokens.
- Keep the basic functionality the same as the existing LP token-based Superfluid staking

