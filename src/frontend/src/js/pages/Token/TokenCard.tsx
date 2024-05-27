import * as React from 'react';
import Skeleton from 'react-loading-skeleton';

import RealEstate, { fromToken } from '../../data/real_estate';
import tokenMetadata from '../../api/tokenMetadata';
import { useAppContext } from '../../components/App/AppContext';
import Container from '../../components/reusable/Container';
import Heading from '../../components/reusable/Heading';
import Address from '../../components/shared/RealEstate/Address';
import Paragraph from '../../components/reusable/Paragraph';
import ListingInfo from './ListingInfo';
import Carousel from '../../components/reusable/Carousel';

interface Props {
  tokenId: bigint;
}

const TokenCard = ({ tokenId }: Props) => {
  const { setAppError } = useAppContext();
  const [token, setToken] = React.useState<RealEstate>();

  React.useEffect(() => {
    if (token !== undefined) {
      return;
    }
    // Fetch token data
    tokenMetadata(tokenId)
      .then((data) => {
        setToken(fromToken(data));
      })
      .catch((e) => {
        console.error(e);
        setAppError('Failed to fetch token data');
      });
  }, []);

  return (
    <Container.Container className="w-full">
      {token ? <TokenCardBody token={token} /> : <SkeletonCard />}
    </Container.Container>
  );
};

const TokenCardBody = ({ token }: { token: RealEstate }) => (
  <Container.Container>
    <Container.FlexResponsiveRow className="text-text gap-8 w-full">
      <Container.Card className="bg-white !p-2 h-fit">
        <img
          loading="lazy"
          src={token.thumbnail}
          alt={token.title}
          width={512}
          height={512}
          className="w-full h-[512px] object-cover"
        />
      </Container.Card>
      <Container.Card className="bg-white flex-1">
        <Container.FlexCols className="gap-2">
          <Heading.H2>{token.title}</Heading.H2>
          <Address realEstate={token} />
          {token.owner && (
            <Container.Container>
              <span className="text-text">
                Owned by <span className="text-blue-500">{token.owner}</span>
              </span>
            </Container.Container>
          )}
          <Container.Container>
            <span className="text-brand text-lg">
              {token.price.toLocaleString('en-US', {
                style: 'currency',
                currency: 'USD',
              })}
            </span>
          </Container.Container>
          <Container.Container>
            <span className="text-text capitalize">
              {token.squareMeters} sqm | {token.rooms} rooms | {token.bathrooms}{' '}
              bathrooms | floor {token.floor} out of {token.totalFloors}
            </span>
          </Container.Container>
          <Container.Container>
            <Paragraph.Default>{token.description}</Paragraph.Default>
          </Container.Container>
          {token.gallery.length > 0 && (
            <Carousel images={token.gallery} className="mb-4" />
          )}
          <ListingInfo realEstate={token} />
        </Container.FlexCols>
      </Container.Card>
    </Container.FlexResponsiveRow>
  </Container.Container>
);

const SkeletonCard = () => (
  <Container.Container>
    <Container.FlexResponsiveRow className="text-text gap-8 w-full">
      <Container.Container className="w-[150px]">
        <Skeleton circle className="h-full" />
      </Container.Container>
      <Container.Container className="w-5/6">
        <Skeleton count={5} className="w-full" />
      </Container.Container>
    </Container.FlexResponsiveRow>
  </Container.Container>
);

export default TokenCard;
