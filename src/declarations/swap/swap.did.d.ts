import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface CanisterInitData {
  'custodians' : Array<Principal>,
  'icp_ledger_canister_id' : Principal,
  'ledger_canister_id' : Principal,
  'sale_royalty' : bigint,
}
export interface Listing {
  'expiration_ns' : bigint,
  'seller' : Account,
  'icp_price' : bigint,
}
export type NftError = { 'UnauthorizedOperator' : null } |
  { 'SelfTransfer' : null } |
  { 'TokenNotFound' : null } |
  { 'UnauthorizedOwner' : null } |
  { 'TxNotFound' : null } |
  { 'SelfApprove' : null } |
  { 'OperatorNotFound' : null } |
  { 'ExistedNFT' : null } |
  { 'OwnerNotFound' : null } |
  { 'Other' : string };
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : SwapError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : SwapError };
export type Result_2 = { 'Ok' : [] | [Listing] } |
  { 'Err' : SwapError };
export type SwapError = { 'Nft' : NftError } |
  { 'IcrcTransferFrom' : TransferFromError } |
  { 'NoSuchListing' : null } |
  { 'InsufficientAllowance' : [bigint, bigint] } |
  { 'InsufficientBalance' : [bigint, bigint] } |
  { 'IcrcTransfer' : TransferError } |
  { 'UnauthorizedCaller' : null } |
  { 'ExpirationTooEarly' : null } |
  { 'ListingExpired' : null } |
  { 'CanisterIsNotOperator' : null } |
  { 'AllowanceExpired' : null } |
  { 'CanisterCall' : [RejectionCode, string] };
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export type TransferFromError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'InsufficientAllowance' : { 'allowance' : bigint } } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface _SERVICE {
  'admin_cycles' : ActorMethod<[], bigint>,
  'admin_set_custodians' : ActorMethod<[Array<Principal>], undefined>,
  'admin_set_ledger_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_sale_royalty' : ActorMethod<[bigint], undefined>,
  'admin_withdraw' : ActorMethod<
    [Principal, [] | [Uint8Array | number[]], bigint],
    Result
  >,
  'buy' : ActorMethod<[bigint, [] | [Uint8Array | number[]]], Result_1>,
  'get_listing' : ActorMethod<[bigint], Result_2>,
  'list' : ActorMethod<
    [bigint, bigint, bigint, [] | [Uint8Array | number[]]],
    Result
  >,
  'unlist' : ActorMethod<[bigint], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
