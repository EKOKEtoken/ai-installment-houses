import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  owner: Principal;
  subaccount: [] | [Uint8Array | number[]];
}
export interface CanisterInitData {
  custodians: Array<Principal>;
  icp_ledger_canister_id: Principal;
  ledger_canister_id: Principal;
  sale_royalty: bigint;
}
export interface HttpRequest {
  url: string;
  method: string;
  body: Uint8Array | number[];
  headers: Array<[string, string]>;
}
export interface HttpResponse {
  body: Uint8Array | number[];
  headers: Array<[string, string]>;
  upgrade: [] | [boolean];
  status_code: number;
}
export interface Listing {
  expiration_ns: bigint;
  seller: Account;
  icp_price: bigint;
}
export type NftError =
  | { UnauthorizedOperator: null }
  | { SelfTransfer: null }
  | { TokenNotFound: null }
  | { UnauthorizedOwner: null }
  | { TxNotFound: null }
  | { SelfApprove: null }
  | { OperatorNotFound: null }
  | { ExistedNFT: null }
  | { OwnerNotFound: null }
  | { Other: string };
export type RejectionCode =
  | { NoError: null }
  | { CanisterError: null }
  | { SysTransient: null }
  | { DestinationInvalid: null }
  | { Unknown: null }
  | { SysFatal: null }
  | { CanisterReject: null };
export type Result = { Ok: null } | { Err: SwapError };
export type Result_1 = { Ok: bigint } | { Err: SwapError };
export type Result_2 = { Ok: [] | [Listing] } | { Err: SwapError };
export type SwapError =
  | { Nft: NftError }
  | { IcrcTransferFrom: TransferFromError }
  | { NoSuchListing: null }
  | { InsufficientAllowance: [bigint, bigint] }
  | { InsufficientBalance: [bigint, bigint] }
  | { IcrcTransfer: TransferError }
  | { UnauthorizedCaller: null }
  | { ExpirationTooEarly: null }
  | { ListingExpired: null }
  | { CanisterIsNotOperator: null }
  | { AllowanceExpired: null }
  | { CanisterCall: [RejectionCode, string] };
export type TransferError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { TemporarilyUnavailable: null }
  | { BadBurn: { min_burn_amount: bigint } }
  | { Duplicate: { duplicate_of: bigint } }
  | { BadFee: { expected_fee: bigint } }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { TooOld: null }
  | { InsufficientFunds: { balance: bigint } };
export type TransferFromError =
  | {
      GenericError: { message: string; error_code: bigint };
    }
  | { TemporarilyUnavailable: null }
  | { InsufficientAllowance: { allowance: bigint } }
  | { BadBurn: { min_burn_amount: bigint } }
  | { Duplicate: { duplicate_of: bigint } }
  | { BadFee: { expected_fee: bigint } }
  | { CreatedInFuture: { ledger_time: bigint } }
  | { TooOld: null }
  | { InsufficientFunds: { balance: bigint } };

export interface Swap {
  admin_cycles: ActorMethod<[], bigint>;
  admin_set_custodians: ActorMethod<[Array<Principal>], undefined>;
  admin_set_ledger_canister: ActorMethod<[Principal], undefined>;
  admin_set_sale_royalty: ActorMethod<[bigint], undefined>;
  admin_withdraw: ActorMethod<
    [Principal, [] | [Uint8Array | number[]], bigint],
    Result
  >;
  buy: ActorMethod<[bigint, [] | [Uint8Array | number[]]], Result_1>;
  get_listing: ActorMethod<[bigint], Result_2>;
  http_request: ActorMethod<[HttpRequest], HttpResponse>;
  list: ActorMethod<
    [bigint, bigint, bigint, [] | [Uint8Array | number[]]],
    Result
  >;
  unlist: ActorMethod<[bigint], Result>;
}

export const idlFactory = ({ IDL }) => {
  const CanisterInitData = IDL.Record({
    custodians: IDL.Vec(IDL.Principal),
    icp_ledger_canister_id: IDL.Principal,
    ledger_canister_id: IDL.Principal,
    sale_royalty: IDL.Nat64,
  });
  const NftError = IDL.Variant({
    UnauthorizedOperator: IDL.Null,
    SelfTransfer: IDL.Null,
    TokenNotFound: IDL.Null,
    UnauthorizedOwner: IDL.Null,
    TxNotFound: IDL.Null,
    SelfApprove: IDL.Null,
    OperatorNotFound: IDL.Null,
    ExistedNFT: IDL.Null,
    OwnerNotFound: IDL.Null,
    Other: IDL.Text,
  });
  const TransferFromError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    InsufficientAllowance: IDL.Record({ allowance: IDL.Nat }),
    BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const TransferError = IDL.Variant({
    GenericError: IDL.Record({
      message: IDL.Text,
      error_code: IDL.Nat,
    }),
    TemporarilyUnavailable: IDL.Null,
    BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }),
    Duplicate: IDL.Record({ duplicate_of: IDL.Nat }),
    BadFee: IDL.Record({ expected_fee: IDL.Nat }),
    CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }),
    TooOld: IDL.Null,
    InsufficientFunds: IDL.Record({ balance: IDL.Nat }),
  });
  const RejectionCode = IDL.Variant({
    NoError: IDL.Null,
    CanisterError: IDL.Null,
    SysTransient: IDL.Null,
    DestinationInvalid: IDL.Null,
    Unknown: IDL.Null,
    SysFatal: IDL.Null,
    CanisterReject: IDL.Null,
  });
  const SwapError = IDL.Variant({
    Nft: NftError,
    IcrcTransferFrom: TransferFromError,
    NoSuchListing: IDL.Null,
    InsufficientAllowance: IDL.Tuple(IDL.Nat, IDL.Nat),
    InsufficientBalance: IDL.Tuple(IDL.Nat, IDL.Nat),
    IcrcTransfer: TransferError,
    UnauthorizedCaller: IDL.Null,
    ExpirationTooEarly: IDL.Null,
    ListingExpired: IDL.Null,
    CanisterIsNotOperator: IDL.Null,
    AllowanceExpired: IDL.Null,
    CanisterCall: IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result = IDL.Variant({ Ok: IDL.Null, Err: SwapError });
  const Result_1 = IDL.Variant({ Ok: IDL.Nat, Err: SwapError });
  const Account = IDL.Record({
    owner: IDL.Principal,
    subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Listing = IDL.Record({
    expiration_ns: IDL.Nat64,
    seller: Account,
    icp_price: IDL.Nat,
  });
  const Result_2 = IDL.Variant({ Ok: IDL.Opt(Listing), Err: SwapError });
  const HttpRequest = IDL.Record({
    url: IDL.Text,
    method: IDL.Text,
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    upgrade: IDL.Opt(IDL.Bool),
    status_code: IDL.Nat16,
  });
  return IDL.Service({
    admin_cycles: IDL.Func([], [IDL.Nat], ['query']),
    admin_set_custodians: IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    admin_set_ledger_canister: IDL.Func([IDL.Principal], [], []),
    admin_set_sale_royalty: IDL.Func([IDL.Nat64], [], []),
    admin_withdraw: IDL.Func(
      [IDL.Principal, IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Nat],
      [Result],
      [],
    ),
    buy: IDL.Func([IDL.Nat, IDL.Opt(IDL.Vec(IDL.Nat8))], [Result_1], []),
    get_listing: IDL.Func([IDL.Nat], [Result_2], ['query']),
    http_request: IDL.Func([HttpRequest], [HttpResponse], ['query']),
    list: IDL.Func(
      [IDL.Nat, IDL.Nat, IDL.Nat64, IDL.Opt(IDL.Vec(IDL.Nat8))],
      [Result],
      [],
    ),
    unlist: IDL.Func([IDL.Nat], [Result], []),
  });
};
