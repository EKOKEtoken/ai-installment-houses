import * as React from 'react';
import { useParams } from 'react-router-dom';
import Skeleton from 'react-loading-skeleton';

import Page from '../components/reusable/Page';
import TokenCard from './Token/TokenCard';

const Token = () => {
  const { tokenId } = useParams();
  const [numTokenId, setNumTokenId] = React.useState<bigint>();

  React.useEffect(() => {
    if (tokenId) {
      setNumTokenId(BigInt(tokenId));
    }
  }, [tokenId]);

  // check token id or redirect
  if (numTokenId === undefined) {
    return (
      <Page.BlankPage>
        <Skeleton count={10} className="block w-[100vw] h-full" />
      </Page.BlankPage>
    );
  }

  return (
    <Page.BlankPage>
      <TokenCard tokenId={BigInt(numTokenId)} />
    </Page.BlankPage>
  );
};

export default Token;
