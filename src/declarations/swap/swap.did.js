export const idlFactory = ({ IDL }) => {
  const Vec = IDL.Rec();
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
    'CanisterCall' : IDL.Tuple(RejectionCode, IDL.Text),
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
  Vec.fill(
    IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Variant({
          'Nat64Content' : IDL.Nat64,
          'Nat32Content' : IDL.Nat32,
          'BoolContent' : IDL.Bool,
          'Nat8Content' : IDL.Nat8,
          'Int64Content' : IDL.Int64,
          'IntContent' : IDL.Int,
          'NatContent' : IDL.Nat,
          'Nat16Content' : IDL.Nat16,
          'Int32Content' : IDL.Int32,
          'Int8Content' : IDL.Int8,
          'FloatContent' : IDL.Float64,
          'Int16Content' : IDL.Int16,
          'BlobContent' : IDL.Vec(IDL.Nat8),
          'NestedContent' : Vec,
          'Principal' : IDL.Principal,
          'TextContent' : IDL.Text,
        }),
      )
    )
  );
  const GenericValue = IDL.Variant({
    'Nat64Content' : IDL.Nat64,
    'Nat32Content' : IDL.Nat32,
    'BoolContent' : IDL.Bool,
    'Nat8Content' : IDL.Nat8,
    'Int64Content' : IDL.Int64,
    'IntContent' : IDL.Int,
    'NatContent' : IDL.Nat,
    'Nat16Content' : IDL.Nat16,
    'Int32Content' : IDL.Int32,
    'Int8Content' : IDL.Int8,
    'FloatContent' : IDL.Float64,
    'Int16Content' : IDL.Int16,
    'BlobContent' : IDL.Vec(IDL.Nat8),
    'NestedContent' : Vec,
    'Principal' : IDL.Principal,
    'TextContent' : IDL.Text,
  });
  const TokenMetadata = IDL.Record({
    'transferred_at' : IDL.Opt(IDL.Nat64),
    'transferred_by' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Opt(IDL.Principal),
    'operator' : IDL.Opt(IDL.Principal),
    'approved_at' : IDL.Opt(IDL.Nat64),
    'approved_by' : IDL.Opt(IDL.Principal),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'is_burned' : IDL.Bool,
    'token_identifier' : IDL.Nat,
    'burned_at' : IDL.Opt(IDL.Nat64),
    'burned_by' : IDL.Opt(IDL.Principal),
    'minted_at' : IDL.Nat64,
    'minted_by' : IDL.Principal,
  });
  const GetListing = IDL.Record({
    'listing' : Listing,
    'metadata' : TokenMetadata,
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Opt(GetListing),
    'Err' : SwapError,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'upgrade' : IDL.Opt(IDL.Bool),
    'status_code' : IDL.Nat16,
  });
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
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
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
