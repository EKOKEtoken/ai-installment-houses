import * as React from 'react';
import RealEstate from '../../../data/real_estate';
import Container from '../../reusable/Container';

interface Props {
  realEstate: RealEstate;
}

const Address = ({ realEstate }: Props) => (
  <Container.Container>
    <span className="text-text">
      {realEstate.address}, {realEstate.civic}, {realEstate.zipCode}{' '}
      {realEstate.city}, {realEstate.country}
    </span>
  </Container.Container>
);

export default Address;
