package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
)

// NewSuperfluidmOsmoIntermediaryAccount returns a new instance of SuperfluidmOsmoIntermediaryAccount.
func NewSuperfluidmOsmoIntermediaryAccount(denom string, valAddr string, gaugeId uint64) SuperfluidmOsmoIntermediaryAccount {
	return SuperfluidmOsmoIntermediaryAccount{
		Denom:   denom,
		ValAddr: valAddr,
		GaugeId: gaugeId,
	}
}

func (a SuperfluidmOsmoIntermediaryAccount) Empty() bool {
	// if intermediary account isn't set in state, we get the default intermediary account.
	// if it set, then the denom is non-blank
	return a.Denom == ""
}

// / NOTE: Importing below two functions from x/superfluid/types/superfluid.go
func (a SuperfluidmOsmoIntermediaryAccount) GetAccAddress() sdk.AccAddress {
	return GetSuperfluidmOsmoIntermediaryAccountAddr(a.Denom, a.ValAddr)
}

func GetSuperfluidmOsmoIntermediaryAccountAddr(denom, valAddr string) sdk.AccAddress {
	// TODO: Make this better namespaced.
	// if ValAddr's one day switch to potentially be 32 bytes, a malleability attack could be crafted.
	// We are launching with the address as is, so this will have to be done as a migration in the future.
	return authtypes.NewModuleAddress(denom + valAddr)
}
