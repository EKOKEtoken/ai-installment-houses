import { ApiListings } from '../data/api';
import { swapJsonRequest } from './api';

const getListings = async (
  offset: number,
  limit: number,
): Promise<ApiListings> => {
  return await swapJsonRequest('get_listings', mock(limit, offset), {
    limit,
    offset,
  });
};

const mock = (limit: number, offset: number): ApiListings => {
  const items = [];
  for (let i = offset; i < limit + limit; i++) {
    items.push({
      listing: {
        seller: {
          owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
          subaccount: [],
        },
        icpPrice: 100_000_000,
        expirationNs: 10000000000,
      },
      metadata: {
        token_identifier: [BigInt(i)],
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
          [
            'gallery',
            {
              NestedContent: [
                ['1', { TextContent: 'https://via.placeholder.com/256' }],
                ['2', { TextContent: 'https://via.placeholder.com/256' }],
                ['3', { TextContent: 'https://via.placeholder.com/256' }],
                ['4', { TextContent: 'https://via.placeholder.com/256' }],
              ],
            },
          ],
        ],
      },
    });
  }

  return {
    listings: items,
    total: 64,
  };
};

export default getListings;
