type TokenProperty = [
  string,
  { TextContent: string } | { NestedContent: TokenProperty[] },
];

export interface ApiTokenMetadata {
  token_identifier: bigint[];
  minted_at: bigint | null;
  minted_by: string | null;
  owner: string | null;
  operator: string | null;
  transferred_at: string | null;
  transferred_by: string | null;
  approved_at: string | null;
  approved_by: string | null;
  is_burned: boolean;
  burned_at: string | null;
  burned_by: string | null;
  properties: TokenProperty[];
}

export interface ApiListing {
  seller: {
    owner: string;
    subaccount: number[];
  };
  icpPrice: number;
  expirationNs: bigint;
}

export interface ApiGetListing {
  listing: ApiListing;
  metadata: ApiTokenMetadata;
}

export interface ApiListings {
  listings: ApiGetListing[];
  total: number;
}
