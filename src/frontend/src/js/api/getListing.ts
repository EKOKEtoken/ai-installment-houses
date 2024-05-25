import { ApiGetListing } from '../data/api';
import { swapJsonRequest } from './api';

const getListing = async (id: bigint): Promise<ApiGetListing | null> => {
  const listing = await swapJsonRequest(
    'get_listing',
    {
      metadata: {
        token_identifier: [id],
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
      },
      listing: {
        seller: {
          owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
          subaccount: [],
        },
        icpPrice: 100_000_000,
        expirationNs: 10000000000,
      },
    },
    { id },
  );

  return listing;
};

export default getListing;
