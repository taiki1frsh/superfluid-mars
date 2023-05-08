package keeper

import (
	"github.com/gogo/protobuf/proto"

	lockuptypes "github.com/osmosis-labs/osmosis/v15/x/lockup/types"
	"github.com/osmosis-labs/osmosis/v15/x/superfluid-mosmo/types"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
)

func (k Keeper) GetAllmOsmoIntermediaryAccounts(ctx sdk.Context) []types.SuperfluidmOsmoIntermediaryAccount {
	store := ctx.KVStore(k.storeKey)
	prefixStore := prefix.NewStore(store, types.KeyPrefixmOsmoIntermediaryAccount)

	accounts := []types.SuperfluidmOsmoIntermediaryAccount{}

	iterator := sdk.KVStorePrefixIterator(prefixStore, nil)
	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		account := types.SuperfluidmOsmoIntermediaryAccount{}
		err := proto.Unmarshal(iterator.Value(), &account)
		if err != nil {
			panic(err)
		}

		accounts = append(accounts, account)
	}
	return accounts
}

func (k Keeper) GetmOsmoIntermediaryAccount(ctx sdk.Context, address sdk.AccAddress) types.SuperfluidmOsmoIntermediaryAccount {
	store := ctx.KVStore(k.storeKey)
	prefixStore := prefix.NewStore(store, types.KeyPrefixmOsmoIntermediaryAccount)

	acc := types.SuperfluidmOsmoIntermediaryAccount{}
	if address == nil {
		return acc
	}

	bz := prefixStore.Get(address)
	if bz == nil {
		return acc
	}
	err := proto.Unmarshal(bz, &acc)
	if err != nil {
		panic(err)
	}
	return acc
}

func (k Keeper) GetmOsmoIntermediaryAccountsForVal(ctx sdk.Context, valAddr sdk.ValAddress) []types.SuperfluidmOsmoIntermediaryAccount {
	accs := k.GetAllmOsmoIntermediaryAccounts(ctx)
	valAccs := []types.SuperfluidmOsmoIntermediaryAccount{}
	for _, acc := range accs {
		if acc.ValAddr != valAddr.String() { // only apply for slashed validator
			continue
		}
		valAccs = append(valAccs, acc)
	}
	return valAccs
}

func (k Keeper) GetOrCreateIntermediaryAccount(ctx sdk.Context, denom, valAddr string) (types.SuperfluidmOsmoIntermediaryAccount, error) {
	accountAddr := types.GetSuperfluidmOsmoIntermediaryAccountAddr(denom, valAddr)
	storeAccount := k.GetmOsmoIntermediaryAccount(ctx, accountAddr)
	// if storeAccount is in state, we return it.
	if !storeAccount.Empty() {
		return storeAccount, nil
	}
	// Otherwise we create the intermediary account.
	// first step, we create the gaugeID
	gaugeID, err := k.ik.CreateGauge(ctx, true, accountAddr, sdk.Coins{}, lockuptypes.QueryCondition{
		LockQueryType: lockuptypes.ByDuration,
		// move this synthetic denom creation to a dedicated function
		Denom:    k.sk.BondDenom(ctx),
		Duration: k.sk.GetParams(ctx).UnbondingTime,
	}, ctx.BlockTime(), 1)
	if err != nil {
		k.Logger(ctx).Error(err.Error())
		return types.SuperfluidmOsmoIntermediaryAccount{}, err
	}

	intermediaryAcct := types.NewSuperfluidmOsmoIntermediaryAccount(denom, valAddr, gaugeID)
	k.SetIntermediaryAccount(ctx, intermediaryAcct)

	// If the intermediary account's address doesn't already have an auth account associated with it,
	// create a new account. We use base accounts, as this is whats done for cosmwasm smart contract accounts.
	// and in the off-chance someone manages to find a bug that forces the account's creation.
	if !k.ak.HasAccount(ctx, intermediaryAcct.GetAccAddress()) {
		k.ak.SetAccount(ctx, authtypes.NewBaseAccount(intermediaryAcct.GetAccAddress(), nil, 0, 0))
	}

	return intermediaryAcct, nil
}

func (k Keeper) SetIntermediaryAccount(ctx sdk.Context, acc types.SuperfluidmOsmoIntermediaryAccount) {
	store := ctx.KVStore(k.storeKey)
	prefixStore := prefix.NewStore(store, types.KeyPrefixmOsmoIntermediaryAccount)

	bz, err := proto.Marshal(&acc)
	if err != nil {
		panic(err)
	}
	prefixStore.Set(acc.GetAccAddress(), bz)
}

func (k Keeper) DeleteIntermediaryAccount(ctx sdk.Context, address sdk.AccAddress) {
	store := ctx.KVStore(k.storeKey)
	prefixStore := prefix.NewStore(store, types.KeyPrefixmOsmoIntermediaryAccount)
	prefixStore.Delete(address)
}
