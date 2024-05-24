export interface ApiTokenMetadata {
  token_identifier: bigint;
  minted_at: bigint;
  minted_by: string;
  owner?: string;
  operator?: string;
  transferred_at?: string;
  transferred_by?: string;
  approved_at?: string;
  approved_by?: string;
  is_burned: boolean;
  burned_at?: string;
  burned_by?: string;
  properties: Array<[string, { TextContent: string }]>;
}

export interface ApiListing {
  id: bigint;
  seller: {
    owner: string;
    subaccount: number[];
  };
  icpPrice: number;
  expirationNs: number;
}

export interface ApiListings {
  listings: ApiListing[];
  total: number;
}
