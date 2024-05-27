import * as React from 'react';

import Container from '../../../components/reusable/Container';
import RealEstate from '../../../data/real_estate';
import { e8sToIcp } from '../../../utils';
import { Route } from '../../../utils/routes';

interface Props {
  token: RealEstate;
}

const ListedToken = ({ token }: Props) => (
  <a href={Route.tokenUrl(token.id)}>
    <Container.Card className="bg-white flex-1 !p-2 transition-transform transform scale-100 hover:scale-105 ">
      <Container.FlexCols className="w-full items-center">
        <img
          loading="lazy"
          src={token.thumbnail}
          alt={token.address}
          width={256}
          height={256}
          className="max-h-[256px] max-w-[256px] w-auto object-cover rounded-lg"
        />
        <span className="text-lg text-brand font-semibold">{token.title}</span>
        <Container.FlexRow className="items-center justify-between w-full px-2">
          <span className="text-xs text-light">
            Token #{token.id.toString()}
          </span>
          <span className="text-brandRed font-semibold">
            {e8sToIcp(BigInt(token.icpPriceE8s!)).toLocaleString('en-US', {
              style: 'currency',
              currency: 'ICP',
              minimumFractionDigits: 2,
              maximumFractionDigits: 8,
            })}
          </span>
        </Container.FlexRow>
      </Container.FlexCols>
    </Container.Card>
  </a>
);

export default ListedToken;
