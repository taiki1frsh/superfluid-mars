package types

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

// constants.
const (
	TypeMsgSuperfluidmOsmoDelegate   = "superfluid_mosmo_delegate"
	TypeMsgSuperfluidmOsmoUndelegate = "superfluid_mosmo_undelegate"
	TypeMsgRebalance                 = "rebalance"
)

var _ sdk.Msg = &MsgSuperfluidmOsmoDelegate{}

// NewMsgSuperfluidmOsmoDelegate creates a message to do superfluid delegation.
func NewMsgSuperfluidmOsmoDelegate(sender sdk.AccAddress, valAddr sdk.ValAddress, amount string) *MsgSuperfluidmOsmoDelegate {
	return &MsgSuperfluidmOsmoDelegate{
		Sender:           sender.String(),
		ValidatorAddress: valAddr.String(),
		Amount:           amount,
	}
}

func (m MsgSuperfluidmOsmoDelegate) Route() string { return RouterKey }
func (m MsgSuperfluidmOsmoDelegate) Type() string  { return TypeMsgSuperfluidmOsmoDelegate }
func (m MsgSuperfluidmOsmoDelegate) ValidateBasic() error {
	if m.Sender == "" {
		return fmt.Errorf("sender should not be an empty address")
	}

	if m.ValidatorAddress == "" {
		return fmt.Errorf("ValAddr should not be empty")
	}
	return nil
}

func (m MsgSuperfluidmOsmoDelegate) GetSignBytes() []byte {
	return sdk.MustSortJSON(ModuleCdc.MustMarshalJSON(&m))
}

func (m MsgSuperfluidmOsmoDelegate) GetSigners() []sdk.AccAddress {
	sender, _ := sdk.AccAddressFromBech32(m.Sender)
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgSuperfluidmOsmoUndelegate{}

// NewMsgSuperfluidmOsmoUndelegate creates a message to do superfluid undelegation.
func NewMsgSuperfluidmOsmoUndelegate(sender sdk.AccAddress, amount string) *MsgSuperfluidmOsmoUndelegate {
	return &MsgSuperfluidmOsmoUndelegate{
		Sender: sender.String(),
		Amount: amount,
	}
}

func (m MsgSuperfluidmOsmoUndelegate) Route() string { return RouterKey }
func (m MsgSuperfluidmOsmoUndelegate) Type() string  { return TypeMsgSuperfluidmOsmoUndelegate }
func (m MsgSuperfluidmOsmoUndelegate) ValidateBasic() error {
	if m.Sender == "" {
		return fmt.Errorf("sender should not be an empty address")
	}

	return nil
}

func (m MsgSuperfluidmOsmoUndelegate) GetSignBytes() []byte {
	return sdk.MustSortJSON(ModuleCdc.MustMarshalJSON(&m))
}

func (m MsgSuperfluidmOsmoUndelegate) GetSigners() []sdk.AccAddress {
	sender, _ := sdk.AccAddressFromBech32(m.Sender)
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgRebalance{}

// NewMsgRebalance creates a message to do rebalance.
func NewMsgRebalance(sender sdk.AccAddress, rebalanceRate sdk.Dec) *MsgRebalance {
	return &MsgRebalance{
		Sender:        sender.String(),
		RebalanceRate: rebalanceRate,
	}
}

func (m MsgRebalance) Route() string { return RouterKey }
func (m MsgRebalance) Type() string  { return TypeMsgRebalance }

func (m MsgRebalance) ValidateBasic() error {
	if m.Sender == "" {
		return fmt.Errorf("sender should not be an empty address")
	}

	return nil
}

func (m MsgRebalance) GetSignBytes() []byte {
	return sdk.MustSortJSON(ModuleCdc.MustMarshalJSON(&m))
}

func (m MsgRebalance) GetSigners() []sdk.AccAddress {
	sender, _ := sdk.AccAddressFromBech32(m.Sender)
	return []sdk.AccAddress{sender}
}
