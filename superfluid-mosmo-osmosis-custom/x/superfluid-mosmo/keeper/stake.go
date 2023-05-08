package keeper

import sdk "github.com/cosmos/cosmos-sdk/types"

// SuperfluidmOsmoDelegate delegates mOsmo to a validator
// NOTE: amount has now strig type because of the error of make proto-gen
// It's intended to be a sdk.Int
func (k Keeper) SuperfluidmOsmoDelegate(ctx sdk.Context, delegator, validator string, amount string) error {
	// amount string to sdk.Int

	// TODO: Properly set the intermediary account connectioni

	// TODO: consider how lock id is treated in this type of superfluid staking
	// Point1. It's used as the very fundamental part of the various kind os the data resources.
	//         So, create lockup data in Contract and pass the id of it to the keeper via msg arg
	//         And, use it as the iminitate data resource for the various kind of the data resources.

	// TODO: maybe this is done by achieving the above TODO.
	// Manage the rewards distribution data resources appropriately

	// TODO: Handle OSMO minting

	// TODO: Handle staking using minted OSMO
	// possibly, we should return the actual amount of the staked OSMO to let the contarct know
	// and update the data for better and more accurate data management

	return nil
}
