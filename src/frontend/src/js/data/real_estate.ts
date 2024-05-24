import { tokenTextProperty } from '../utils';
import { ApiListing, ApiTokenMetadata } from './api';

class RealEstate {
  public id: bigint;
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
  public listing: ApiListing | null;

  constructor(token: ApiTokenMetadata, listing: ApiListing | null) {
    this.id = token.token_identifier;
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
    this.listing = listing;
  }
}

export default RealEstate;
