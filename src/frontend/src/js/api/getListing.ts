import { ApiListing } from '../data/api';
import { swapJsonRequest } from './api';

const getListing = async (id: bigint): Promise<ApiListing | null> => {
  const listing = await swapJsonRequest(
    'get_listing',
    {
      listing: {
        id,
        seller: {
          owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
          subaccount: [],
        },
        icpPrice: 100,
        expirationNs: 10000000000,
      },
    },
    { id },
  );

  return listing.listing;
};

export default getListing;
