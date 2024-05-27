import { e8sToIcpNumber, tokenTextProperty } from '../utils';
import { ApiGetListing, ApiTokenMetadata } from './api';

interface RealEstate {
  id: bigint;
  title: string;
  description: string;
  country: string;
  city: string;
  address: string;
  owner: string | null;
  civic: string;
  zipCode: string;
  floor: string;
  totalFloors: string;
  squareMeters: string;
  rooms: string;
  bathrooms: string;
  price: number;
  thumbnail: string;
  gallery: string[];
  seller: string | null;
  icpPriceE8s: number | null;
  icpPriceView: number | null;
  expiration: Date | null;
}

export const fromToken = (
  token: ApiTokenMetadata | ApiGetListing,
): RealEstate => {
  if ('listing' in token) {
    return {
      id: token.metadata.token_identifier[0]!,
      owner: token.metadata.owner,
      title: tokenTextProperty(token.metadata, 'title') || '',
      description: tokenTextProperty(token.metadata, 'description') || '',
      country: tokenTextProperty(token.metadata, 'country') || '',
      city: tokenTextProperty(token.metadata, 'city') || '',
      address: tokenTextProperty(token.metadata, 'address') || '',
      civic: tokenTextProperty(token.metadata, 'civic') || '',
      zipCode: tokenTextProperty(token.metadata, 'zipCode') || '',
      floor: tokenTextProperty(token.metadata, 'floor') || '',
      totalFloors: tokenTextProperty(token.metadata, 'totalFloors') || '',
      squareMeters: tokenTextProperty(token.metadata, 'squareMeters') || '',
      rooms: tokenTextProperty(token.metadata, 'rooms') || '',
      bathrooms: tokenTextProperty(token.metadata, 'bathrooms') || '',
      price: parseInt(tokenTextProperty(token.metadata, 'price') || ''),
      thumbnail: tokenTextProperty(token.metadata, 'thumbnail') || '',
      gallery: getGalleryFromProps(token.metadata),
      seller: token.listing.seller.owner,
      icpPriceE8s: token.listing.icpPrice,
      icpPriceView: e8sToIcpNumber(token.listing.icpPrice),
      expiration: new Date(
        Number(token.listing.expirationNs / BigInt(1_000_000)),
      ),
    };
  } else {
    return {
      id: token.token_identifier[0]!,
      owner: token.owner,
      title: tokenTextProperty(token, 'title') || '',
      description: tokenTextProperty(token, 'description') || '',
      country: tokenTextProperty(token, 'country') || '',
      city: tokenTextProperty(token, 'city') || '',
      address: tokenTextProperty(token, 'address') || '',
      civic: tokenTextProperty(token, 'civic') || '',
      zipCode: tokenTextProperty(token, 'zipCode') || '',
      floor: tokenTextProperty(token, 'floor') || '',
      totalFloors: tokenTextProperty(token, 'totalFloors') || '',
      squareMeters: tokenTextProperty(token, 'squareMeters') || '',
      rooms: tokenTextProperty(token, 'rooms') || '',
      bathrooms: tokenTextProperty(token, 'bathrooms') || '',
      price: parseInt(tokenTextProperty(token, 'price') || ''),
      thumbnail: tokenTextProperty(token, 'thumbnail') || '',
      gallery: getGalleryFromProps(token),
      seller: null,
      icpPriceE8s: null,
      icpPriceView: null,
      expiration: null,
    };
  }
};

export const setListing = (
  realEstate: RealEstate,
  listing: ApiGetListing,
): RealEstate => {
  return {
    ...realEstate,
    seller: listing.listing.seller.owner,
    icpPriceE8s: listing.listing.icpPrice,
    icpPriceView: e8sToIcpNumber(listing.listing.icpPrice),
    expiration: new Date(
      Number(listing.listing.expirationNs / BigInt(1_000_000)),
    ),
  };
};

const getGalleryFromProps = (token: ApiTokenMetadata): string[] => {
  const gallery = token.properties.find(
    (property) => property[0] === 'gallery',
  );
  if (gallery) {
    const genericValue = gallery[1];
    // check if generic value is `{ NestedContent: TokenProperty[] }`
    if ('NestedContent' in genericValue) {
      return genericValue.NestedContent.map((galleryItem) => {
        if ('TextContent' in galleryItem[1]) {
          const galleryItemUrl = galleryItem[1].TextContent;
          return galleryItemUrl;
        } else {
          return null;
        }
      }).filter((galleryItem) => galleryItem !== null) as string[];
    }
  }

  return [];
};

export default RealEstate;
