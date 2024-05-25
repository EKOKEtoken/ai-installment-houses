import { tokenTextProperty } from '../utils';
import { ApiGetListing, ApiTokenMetadata } from './api';

class RealEstate {
  public id: bigint;
  public title: string;
  public country: string;
  public city: string;
  public address: string;
  public civic: string;
  public zipCode: string;
  public floor: string;
  public totalFloors: string;
  public squareMeters: string;
  public rooms: string;
  public bathrooms: string;
  public price: string;
  public thumbnail: string;
  // listing
  public seller: string | null;
  public icpPrice: number | null;
  public expirationNs: number | null;

  constructor(token: ApiTokenMetadata | ApiGetListing) {
    if ('listing' in token) {
      this.id = token.metadata.token_identifier[0]!;
      this.title = tokenTextProperty(token.metadata, 'title') || '';
      this.country = tokenTextProperty(token.metadata, 'country') || '';
      this.city = tokenTextProperty(token.metadata, 'city') || '';
      this.address = tokenTextProperty(token.metadata, 'address') || '';
      this.civic = tokenTextProperty(token.metadata, 'civic') || '';
      this.zipCode = tokenTextProperty(token.metadata, 'zipCode') || '';
      this.floor = tokenTextProperty(token.metadata, 'floor') || '';
      this.totalFloors = tokenTextProperty(token.metadata, 'totalFloors') || '';
      this.squareMeters =
        tokenTextProperty(token.metadata, 'squareMeters') || '';
      this.rooms = tokenTextProperty(token.metadata, 'rooms') || '';
      this.bathrooms = tokenTextProperty(token.metadata, 'bathrooms') || '';
      this.price = tokenTextProperty(token.metadata, 'price') || '';
      this.thumbnail = tokenTextProperty(token.metadata, 'thumbnail') || '';
      this.seller = token.listing.seller.owner;
      this.icpPrice = token.listing.icpPrice;
      this.expirationNs = token.listing.expirationNs;
    } else {
      this.id = token.token_identifier[0]!;
      this.title = tokenTextProperty(token, 'title') || '';
      this.country = tokenTextProperty(token, 'country') || '';
      this.city = tokenTextProperty(token, 'city') || '';
      this.address = tokenTextProperty(token, 'address') || '';
      this.civic = tokenTextProperty(token, 'civic') || '';
      this.zipCode = tokenTextProperty(token, 'zipCode') || '';
      this.floor = tokenTextProperty(token, 'floor') || '';
      this.totalFloors = tokenTextProperty(token, 'totalFloors') || '';
      this.squareMeters = tokenTextProperty(token, 'squareMeters') || '';
      this.rooms = tokenTextProperty(token, 'rooms') || '';
      this.bathrooms = tokenTextProperty(token, 'bathrooms') || '';
      this.price = tokenTextProperty(token, 'price') || '';
      this.thumbnail = tokenTextProperty(token, 'thumbnail') || '';
      this.seller = null;
      this.icpPrice = null;
      this.expirationNs = null;
    }
  }
}

export default RealEstate;
