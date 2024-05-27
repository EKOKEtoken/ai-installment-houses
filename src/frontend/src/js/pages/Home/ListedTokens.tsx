import * as React from 'react';

import getListings from '../../api/getListings';
import Container from '../../components/reusable/Container';
import RealEstate, { fromToken } from '../../data/real_estate';
import ListedToken from './ListedTokens/ListedToken';

const ListedTokens = () => {
  const [listedTokens, setListedTokens] = React.useState<RealEstate[]>();

  React.useEffect(() => {
    getListings(0, 20).then((data) => {
      const estates = data.listings.map((item) => fromToken(item));
      setListedTokens(estates);
    });
  });

  if (!listedTokens) {
    return null;
  }

  const items = listedTokens.map((item) => (
    <ListedToken key={item.id} token={item} />
  ));

  return (
    <Container.Container className="grid sm:flex sm:flex-col grid-cols-4 gap-8">
      {items}
    </Container.Container>
  );
};

export default ListedTokens;
