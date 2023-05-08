package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/osmosis-labs/osmosis/v15/x/superfluid-mosmo/types"
)

type msgServer struct {
	keeper *Keeper
}

// NewMsgServerImpl returns an instance of MsgServer.
func NewMsgServerImpl(keeper *Keeper) types.MsgServer {
	return &msgServer{
		keeper: keeper,
	}
}

var _ types.MsgServer = msgServer{}

func (server msgServer) SuperfluidmOsmoDelegate(goCtx context.Context, msg *types.MsgSuperfluidmOsmoDelegate) (*types.MsgSuperfluidmOsmoDelegateResponse, error) {
	_ = sdk.UnwrapSDKContext(goCtx)

	// err := server.keeper.SuperfluidmOsmoDelegate(ctx, msg.Sender, msg.ValidatorAddress, msg.Amount)
	// if err == nil {
	// 	events.EmitSuperfluidDelegateEvent(ctx, msg.LockId, msg.ValAddr)
	// }
	return &types.MsgSuperfluidmOsmoDelegateResponse{}, nil
}

// SuperfluidUndelegate undelegates currently superfluid delegated position.
// Old synthetic lock is deleted and a new synthetic lock is created to indicate the unbonding position.
// The actual staking position is instantly undelegated and the undelegated tokens are instantly sent from
// the intermediary account to the module account.
// Note that SuperfluidUndelegation does not start unbonding of the underlying lock iteslf.
func (server msgServer) SuperfluidmOsmoUndelegate(goCtx context.Context, msg *types.MsgSuperfluidmOsmoUndelegate) (*types.MsgSuperfluidmOsmoUndelegateResponse, error) {
	_ = sdk.UnwrapSDKContext(goCtx)

	// err := server.keeper.SuperfluidmOsmoUndelegate(ctx, msg.Sender, msg.LockId)
	// if err == nil {
	// 	events.EmitSuperfluidUndelegateEvent(ctx, msg.LockId)
	// }
	return &types.MsgSuperfluidmOsmoUndelegateResponse{}, nil
}

// Rebalance re-blances the superfluid staking amount for mOsmos stakers.
func (server msgServer) Rebalance(goCtx context.Context, msg *types.MsgRebalance) (*types.MsgRebalanceResponse, error) {
	_ = sdk.UnwrapSDKContext(goCtx)

	// err := server.keeper.Rebalance(ctx, msg.Sender)
	// if err == nil {
	// 	events.EmitRebalanceEvent(ctx)
	// }
	return &types.MsgRebalanceResponse{}, nil
}
