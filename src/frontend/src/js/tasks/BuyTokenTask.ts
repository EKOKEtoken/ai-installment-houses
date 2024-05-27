import { ActorSubclass } from '@dfinity/agent';

import { IcpLedger } from '../ic/icp-ledger.did';
import { Swap } from '../ic/swap.did';
import { Result, Task } from '../components/shared/TaskList';
import RealEstate from '../data/real_estate';
import { ApiListing } from '../data/api';
import { SWAP_CANISTER_ID } from '../ic/AgentContext';
import { Principal } from '@dfinity/principal';

const buyTokenTasks = (
  token: RealEstate,
  listing: ApiListing,
  swap: ActorSubclass<Swap>,
  icpLedger: ActorSubclass<IcpLedger>,
): Task[] => {
  return [
    {
      label: 'Approve swap for spending ICP',
      action: async () => {
        return await approveSwapForSpending(listing, icpLedger);
      },
    },
    {
      label: 'Buying token from swap canister',
      action: async () => {
        return await buyToken(token, swap);
      },
    },
  ];
};

const getIcpLedgerFee = async (
  icpLedger: ActorSubclass<IcpLedger>,
): Promise<bigint> => {
  return await icpLedger.icrc1_fee();
};

const approveSwapForSpending = async (
  listing: ApiListing,
  icpLedger: ActorSubclass<IcpLedger>,
): Promise<Result> => {
  const icpLedgerFee = await getIcpLedgerFee(icpLedger);
  const approvalAmount = BigInt(listing.icpPrice) + icpLedgerFee + icpLedgerFee;

  const res = await icpLedger.icrc2_approve({
    amount: approvalAmount,
    spender: {
      owner: Principal.fromText(SWAP_CANISTER_ID),
      subaccount: [],
    },
    fee: [],
    from_subaccount: [],
    created_at_time: [],
    expected_allowance: [],
    expires_at: [],
    memo: [],
  });

  if ('Ok' in res) {
    return true;
  } else {
    return { error: `Failed to approve swap for spending: ${res.Err}` };
  }
};

const buyToken = async (
  token: RealEstate,
  swap: ActorSubclass<Swap>,
): Promise<Result> => {
  const res = await swap.buy(token.id, []);

  if ('Ok' in res) {
    return true;
  } else {
    return { error: `Failed to buy token: ${res.Err}` };
  }
};

export default buyTokenTasks;
