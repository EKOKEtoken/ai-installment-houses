import { ApiListings } from '../data/api';
import { swapJsonRequest } from './api';

const getListings = async (
  limit: number,
  offset: number,
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
      id: BigInt(1),
      seller: {
        owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
        subaccount: [],
      },
      icpPrice: 100,
      expirationNs: 10000000000,
    });
  }

  return {
    listings: items,
    total: 64,
  };
};

export default getListings;
