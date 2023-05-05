# Tokenized Mars Protocol

To enable the deposited OSMO on Mars to be staked via superfluid module, I propose the tokenization of the deposited asset like AToken on Aave.

By this feature, the management of the deposited asset will be way eaiser and more composable with the other protocols, like Superfluid Stakind possibly.

To achieve this, the following properties are required as far as I'm concerned at the moment:

- Issue mTokens to the depositor for the amount of tokens deposited
- Distribute interest proportionally based on the mToken share ratio
- Redeem deposited assets by burning mTokens
- Control the reward distribution through mToken burning during liquidation

**Those requirements are almost all written in the code in the form of the TODO comment for now.**
(I don't refer to debt token at all right now unlike Aave, for the record.)

NOTE: My commits can be seen after the "import red-bank".
