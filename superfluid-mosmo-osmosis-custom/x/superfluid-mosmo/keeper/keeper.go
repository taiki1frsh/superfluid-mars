package keeper

import (
	"fmt"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/osmosis-labs/osmosis/v15/x/superfluid-mosmo/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"

	// govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

// Keeper provides a way to manage module storage.
type Keeper struct {
	storeKey   sdk.StoreKey
	paramSpace paramtypes.Subspace

	ak authkeeper.AccountKeeper
	bk types.BankKeeper
	sk types.StakingKeeper
	// CommunityPoolKeeper maybe not needed
	ck types.CommunityPoolKeeper
	// EpochKeeper maybe not needed
	ek types.EpochKeeper
	lk types.LockupKeeper
	// IncentivesKeeper maybe not needed
	ik types.IncentivesKeeper

	// lms types.LockupMsgServer
}

// var _ govtypes.StakingKeeper = (*Keeper)(nil)

// NewKeeper returns an instance of Keeper.
func NewKeeper(storeKey sdk.StoreKey, paramSpace paramtypes.Subspace, ak authkeeper.AccountKeeper, bk types.BankKeeper, sk types.StakingKeeper, dk types.CommunityPoolKeeper, ek types.EpochKeeper, lk types.LockupKeeper, ik types.IncentivesKeeper) *Keeper {
	// set KeyTable if it has not already been set
	if !paramSpace.HasKeyTable() {
		paramSpace = paramSpace.WithKeyTable(types.ParamKeyTable())
	}

	return &Keeper{
		storeKey:   storeKey,
		paramSpace: paramSpace,
		ak:         ak,
		bk:         bk,
		sk:         sk,
		ck:         dk,
		ek:         ek,
		lk:         lk,
		ik:         ik,

		// lms: lms,
	}
}

// Logger returns a logger instance.
func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}
