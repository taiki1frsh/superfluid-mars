package types

import (
	fmt "fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

// Parameter store keys.
var (
	KeyRebalancer     = []byte("Rebalancer")
	defaultRebalancer = ""
)

// ParamTable for minting module.
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

func NewParams(rebalancer string) Params {
	return Params{
		Rebalancer: rebalancer,
	}
}

// default minting module parameters.
func DefaultParams() Params {
	return Params{
		Rebalancer: defaultRebalancer,
	}
}

// validate params.
func (p Params) Validate() error {
	return nil
}

// Implements params.ParamSet.
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyRebalancer, &p.Rebalancer, ValidateRebalancer),
	}
}

func ValidateRebalancer(i interface{}) error {
	v, ok := i.(string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	// check if the string can be converted to sdk.AccAddress
	_, err := sdk.AccAddressFromBech32(v)
	if err != nil {
		return fmt.Errorf("invalid rebalancer address: %s", v)
	}

	return nil
}
