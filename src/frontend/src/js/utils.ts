import { Principal } from '@dfinity/principal';

import { TokenMetadata } from './ic/ledger.did';
import { ApiTokenMetadata } from './data/api';

const DECIMALS = 8;

export const e8sToIcpStr = (e8s: bigint | number): string => {
  // put comma in `decimals` position
  const supplyStr = e8s.toString();
  const arr = supplyStr.split('');
  // check if supply is less than `DECIMALS`
  if (arr.length <= DECIMALS) {
    arr.splice(arr.length - DECIMALS, 0, '');
  } else {
    arr.splice(arr.length - DECIMALS, 0, '.');
  }

  return arr.join('');
};

export const e8sToIcpNumber = (e8s: number): number => {
  return e8s / 10 ** DECIMALS;
};

export const e8sToIcp = (e8s: bigint): bigint => {
  return e8s / 10n ** BigInt(DECIMALS);
};

export const validatePrincipal = (
  principal: string | number | readonly string[] | undefined,
): boolean => {
  if (typeof principal !== 'string') return false;

  try {
    Principal.fromText(principal);
    return true;
  } catch (_) {
    return false;
  }
};
export const validateIsNumber = (
  amount: string | number | readonly string[] | undefined,
): boolean => {
  if (typeof amount !== 'string') return false;

  return !isNaN(parseInt(amount));
};

export const principalToBytes32 = (principal: Principal): string => {
  const principalToHex = principal.toHex();
  // right align the hex string
  return '0x' + principalToHex.padEnd(64, '0');
};

export const tokenTextProperty = (
  token: TokenMetadata | ApiTokenMetadata,
  key: string,
): string | undefined => {
  const prop = token.properties.find((property) => property[0] === key);
  if (prop) {
    const genericValue = prop[1];
    // check if generic value is `{ TextContent: string }`
    if ('TextContent' in genericValue) {
      return genericValue.TextContent;
    }
  }

  return undefined;
};

export const iso3166Name = (iso3166: string): string | undefined => {
  const locale = navigator.language || 'en-US';
  return new Intl.DisplayNames([locale], { type: 'region' }).of(iso3166);
};
