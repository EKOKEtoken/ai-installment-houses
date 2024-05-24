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
  seller: {
    owner: string;
    subaccount: number[];
  };
  icpPrice: number;
  expirationNs: number;
}

export interface ApiGetListing {
  listing: ApiListing;
  metadata: ApiTokenMetadata;
}

export interface ApiListings {
  listings: ApiGetListing[];
  total: number;
}
