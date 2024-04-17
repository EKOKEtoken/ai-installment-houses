export const idlFactory = ({ IDL }) => {
  const CanisterInitData = IDL.Record({
    'custodians' : IDL.Vec(IDL.Principal),
    'icp_ledger_canister_id' : IDL.Principal,
    'ledger_canister_id' : IDL.Principal,
    'sale_royalty' : IDL.Nat64,
  });
  const NftError = IDL.Variant({
    'UnauthorizedOperator' : IDL.Null,
    'SelfTransfer' : IDL.Null,
    'TokenNotFound' : IDL.Null,
    'UnauthorizedOwner' : IDL.Null,
    'TxNotFound' : IDL.Null,
    'SelfApprove' : IDL.Null,
    'OperatorNotFound' : IDL.Null,
    'ExistedNFT' : IDL.Null,
    'OwnerNotFound' : IDL.Null,
    'Other' : IDL.Text,
  });
  const TransferFromError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'InsufficientAllowance' : IDL.Record({ 'allowance' : IDL.Nat }),
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const TransferError = IDL.Variant({
    'GenericError' : IDL.Record({
      'message' : IDL.Text,
      'error_code' : IDL.Nat,
    }),
    'TemporarilyUnavailable' : IDL.Null,
    'BadBurn' : IDL.Record({ 'min_burn_amount' : IDL.Nat }),
    'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
    'BadFee' : IDL.Record({ 'expected_fee' : IDL.Nat }),
    'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
    'TooOld' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : IDL.Nat }),
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const SwapError = IDL.Variant({
    'Nft' : NftError,
    'IcrcTransferFrom' : TransferFromError,
    'NoSuchListing' : IDL.Null,
    'InsufficientAllowance' : IDL.Tuple(IDL.Nat, IDL.Nat),
    'InsufficientBalance' : IDL.Tuple(IDL.Nat, IDL.Nat),
    'IcrcTransfer' : TransferError,
    'UnauthorizedCaller' : IDL.Null,
    'ExpirationTooEarly' : IDL.Null,
    'ListingExpired' : IDL.Null,
    'CanisterIsNotOperator' : IDL.Null,
    'AllowanceExpired' : IDL.Null,
    'CanisterCall' : RejectionCode,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : SwapError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : SwapError });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Listing = IDL.Record({
    'expiration_ns' : IDL.Nat64,
    'seller' : Account,
    'icp_price' : IDL.Nat,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Opt(Listing), 'Err' : SwapError });
  return IDL.Service({
    'admin_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'admin_set_custodians' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'admin_set_ledger_canister' : IDL.Func([IDL.Principal], [], []),
    'admin_set_sale_royalty' : IDL.Func([IDL.Nat64], [], []),
    'admin_withdraw' : IDL.Func(
        [IDL.Principal, IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Nat],
        [Result],
        [],
      ),
    'buy' : IDL.Func([IDL.Nat, IDL.Opt(IDL.Vec(IDL.Nat8))], [Result_1], []),
    'get_listing' : IDL.Func([IDL.Nat], [Result_2], ['query']),
    'list' : IDL.Func(
        [IDL.Nat, IDL.Nat, IDL.Nat64, IDL.Opt(IDL.Vec(IDL.Nat8))],
        [Result],
        [],
      ),
    'unlist' : IDL.Func([IDL.Nat], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const CanisterInitData = IDL.Record({
    'custodians' : IDL.Vec(IDL.Principal),
    'icp_ledger_canister_id' : IDL.Principal,
    'ledger_canister_id' : IDL.Principal,
    'sale_royalty' : IDL.Nat64,
  });
  return [CanisterInitData];
};
