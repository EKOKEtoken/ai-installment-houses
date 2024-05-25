import { ledgerJsonRequest } from './api';
import { ApiTokenMetadata } from '../data/api';

export interface SearchParams {
  offset: number;
  limit: number;
  sortBy: 'id' | 'mintedAt';
  properties: {
    owner?: string;
    country?: string;
    city?: string;
    address?: string;
    civic?: string;
    zipCode?: string;
  };
}

const search = async (params: SearchParams): Promise<ApiTokenMetadata[]> => {
  return await ledgerJsonRequest('search', mock(params), params);
};

const mock = (params: SearchParams): ApiTokenMetadata[] => {
  const metadata: ApiTokenMetadata[] = [];
  for (let i = 1; i <= params.limit; i++) {
    metadata.push({
      token_identifier: [BigInt(i + params.offset)],
      minted_at: BigInt(new Date().getTime() * 1000),
      minted_by: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
      owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
      is_burned: false,
      burned_by: null,
      burned_at: null,
      operator: null,
      transferred_by: null,
      transferred_at: null,
      approved_at: null,
      approved_by: null,
      properties: [
        ['title', { TextContent: 'Trilocale in via Marangoni' }],
        ['country', { TextContent: 'IT' }],
        ['city', { TextContent: 'Udine' }],
        ['address', { TextContent: 'Via Antonio Marangoni' }],
        ['civic', { TextContent: '33' }],
        ['zipCode', { TextContent: '33100' }],
        ['floor', { TextContent: '1' }],
        ['totalFloors', { TextContent: '3' }],
        ['squareMeters', { TextContent: '100' }],
        ['rooms', { TextContent: '3' }],
        ['bathrooms', { TextContent: '2' }],
        ['price', { TextContent: '1000000' }],
        ['thumbnail', { TextContent: 'https://via.placeholder.com/256' }],
      ],
    });
  }
  return metadata;
};

export default search;
