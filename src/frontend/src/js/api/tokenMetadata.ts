import { ledgerJsonRequest } from './api';
import { ApiTokenMetadata } from '../data/api';

const tokenMetadata = async (id: bigint): Promise<ApiTokenMetadata> => {
  return await ledgerJsonRequest('dip721_token_metadata', mock(id), {
    id,
  });
};

const mock = (id: bigint): ApiTokenMetadata => {
  return {
    token_identifier: id,
    minted_at: BigInt(new Date().getTime() * 1000),
    minted_by: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
    owner: 'rwlgt-iiaaa-aaaaa-aaaaa-cai',
    is_burned: false,
    properties: [
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
    ],
  };
};

export default tokenMetadata;
