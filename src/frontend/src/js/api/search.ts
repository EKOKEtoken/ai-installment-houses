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
        [
          'description',
          {
            TextContent:
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum iaculis viverra condimentum. Mauris pellentesque arcu et ultrices consectetur. Nulla non fermentum massa. Phasellus sodales sodales laoreet. Morbi faucibus elit quis lacus congue aliquam. Fusce commodo diam sed viverra pharetra. Curabitur non felis non arcu rutrum viverra vel eget ante. Ut sagittis sapien neque, sed porttitor metus finibus in. Nunc venenatis lobortis pharetra. Proin ultricies, mauris quis euismod mattis, quam felis suscipit nunc, vitae rhoncus sem enim vel neque. Mauris turpis purus, efficitur vestibulum metus nec, tincidunt ultricies dolor. Donec a gravida augue, nec aliquet sapien. Suspendisse at elementum erat, nec blandit tellus. Donec id pulvinar tortor. Mauris sed ultrices elit, sed sagittis nulla. Curabitur purus ipsum, fringilla non accumsan non, ultricies non metus. Maecenas ut commodo mauris. Duis suscipit pellentesque ligula eu consectetur. Donec commodo ligula at vehicula porta. Aenean ornare, nulla nec faucibus efficitur, ex leo suscipit enim, nec accumsan nunc erat at elit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aenean nec lectus in nibh consequat fermentum vitae ac felis. Nulla vitae dui ut enim blandit sagittis. Curabitur volutpat porta mi, convallis malesuada est pellentesque eu. Integer facilisis risus id auctor mattis. Duis sit amet congue metus. Etiam finibus ac nunc eu faucibus. Nullam mattis nulla ex, et molestie odio efficitur non. Integer aliquet, diam non rhoncus pellentesque, libero dolor mollis risus, non tincidunt ligula massa at mauris. Etiam vitae pharetra lorem, porttitor tempor dolor.',
          },
        ],
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
        [
          'thumbnail',
          { TextContent: 'https://dummyimage.com/512x512/eee/aaa' },
        ],
        [
          'gallery',
          {
            NestedContent: [
              ['1', { TextContent: 'https://dummyimage.com/1024x768/eee/aaa' }],
              ['2', { TextContent: 'https://dummyimage.com/1024x768/eee/aaa' }],
              ['3', { TextContent: 'https://dummyimage.com/1024x768/eee/aaa' }],
              ['4', { TextContent: 'https://dummyimage.com/1024x768/eee/aaa' }],
            ],
          },
        ],
      ],
    });
  }
  return metadata;
};

export default search;
