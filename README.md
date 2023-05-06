# Osmosis Superfluid Staking **on Mars**

Make $OSMO that is deposited on Mars stakable via Superfluid Staking by leveraging super flexible design capacity of CosmWasm and Cosmos SDK.

## Concept

The simplest way to describe the goal is to **enable **deposited OSMO **on** Mars to** be** staked.****

Looking back at the revolutionary idea of Superfluid Staking as an App Chain, it was quite simple at its core, seeing the potential liquidity that exists locked on-chain but is not being used efficiently as a kind of staking state. In the case of Osmosis, this was $OSMO that was provided as liquidity in constant product AMM pools. 　　
As time passed, the Osmosis ecosystem diversified, and so did the ways of providing liquidity. A prime example of this is the Mars Protocol.

In Mars Protocol, there are always more than a certain amount of $OSMO tokens being lent but not used as collateral. This is due to the actions of users who aim to earn interest through lending, as well as the actions of passive borrowers who only borrow a small number of tokens against their collateral.

The nature of these tokens is very similar to that of $OSMO, which is provided as liquidity in constant product AMM pools but is practically unexchangeable due to being in a price range where no trades can occur. Based on this, I believe that it is possible and beneficial to create a staking state for some of the $OSMO collateral in the Mars Protocol through a mechanism similar to Superfluid Staking.

## Core Designs

Casually describing, I believe `superfluid mars` can be achieved by the following features:

1. Tokenize bonds to know the amount of Osmosis being lent and to improve composability.

- Issuance of mOSMO (CW20)
- (Issuance of debt tokens (CW20))

2. (Low priority) Enable collateral management based on the presence or absence of bonds. For all transactions involving claims, use bond tokens.

- Mint of mToken when depositing
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
- Possibly, design the module so that it can support the other bond tokens from Mars

### Bonding

LP Share TokenによるSuperfluid Stakingと違い、mOsmoの価値は常にOSMOと1:1であるため、ユーザーが指定するmOSMOのBonding Amountと、実際にSuperfluidモジュールを通してBondingされるOSMOの量は同じである。mOSMOのロックアップは、`superfluid-mosmo-staking` contractによって、そのコントラクトアドレス上で行われる。
ユーザーによるValidatorの指定は今回の実装では行わないが、現x/superfluidモジュールとの統合を行う上で、Intermediary accountによるManagementで対応することに大きな障壁はないと考えられる。


### Unbonding

### Rebalance

- もし、Utilization Rateから算出される最大Staking可能量を超えた場合、Share割合に応じて、 Staking量を減らす必要がある。　　　
- だが、報酬の分配比率は保持できるから、大した問題にはならないか？

#### Claim the staking reward

- 各Superfluid mOsmo参加者のmOSMOロック残高に応じて、報酬の分配割合を決定する。
- これはContractで行う必要がある。　　
- DelegatingしているValidstorごとに報酬の量は異なるため、それぞれの報酬に応じた割合を決定する必要がある。

## Technical Design

### Tokenized Red-Bank

未使用のOSMO担保をSuperfluidを通してStakingすることを可能にするために、まずLending機能のTokenizationを提案したい。
これはAave, Compoundなどの先行事例があることから想像しやすいと思うが、単純に言えば担保の量を表す合成トークンの発行、さらに債務を表す合成トークンの発行により、金利、貸し借りの状態を全てそれらのトークンを通して表現しようというものである。
今回の実装では、特にSuperfluid Stakingと関連する、担保のトークンのみの実装を考える。（もちろん、債務トークンとの兼ね合いは考えているが、今回の実装では割愛する。）

### mToken

Introduce mToken as a collateral token for Mars Protocol. This corresponds 1:1 with tokens deposited in Red-Bank, and the total issuance of mTokens is equal to the total amount of tokens deposited in Red-Bank.   

Its features include:

- mToken is a CW20
- mToken corresponds 1:1 with tokens deposited in Red-Bank
- mToken holds UnusableAmount data, which restricts free transfers without requiring lockup to a specific contract
- By using UnusableAmount, it is possible to properly distribute interest by realizing the lockup of mOSMO in the Superfluid contract without transferring it to the contract
- UnusableAmount can only be controlled by the contract address that holds the data
- mToken also uses UnusableAmount to handle lockup of collateral due to borrowing
- Collateral redemption can only be performed by the mToken holder at the time of redemption, regardless of the original mToken issuer
- mToken can be basically transferred freely, and can also be controlled by contracts through Allowance as normal CW20

### Superfluid mOSMO Module

### Superfluid mOSMO Contract

In this contract, we provide functions that correspond to the control part in the x/superfluid module where Superfluid Staking is controlled and executed by LP share tokens. Specifically, this contract performs the pseudo lockup of the unused portion of mOSMO as collateral using UnusableAmount, and calculates the maximum amount of mOSMO that can be staked in Superfluid Staking based on the usage rate of OSMO in Red-Bank.

It is not possible and should be avoided to introduce all unused mOSMO into Superfluid Staking. This is because if the usage rate of mOSMO becomes 100% due to Superfluid Staking, the usage rate of OSMO in Red-Bank will also become 100%, making debt repayment in Red-Bank impossible.

Therefore, it is desirable to introduce a certain proportion of the unused mOSMO into Superfluid Staking. This proportion is managed and updated as Config data held by this contract. The distribution of staking rewards is maintained according to the locked mOSMO amount associated with each address managed by this contract. Distribution occurs only through data during Epochs, either by releasing it through a Claim Msg or by actually sending tokens.

Unbonding is realized by releasing the UnusableAmount of mOSMO after a period similar to the normal OSMO unbonding period.
