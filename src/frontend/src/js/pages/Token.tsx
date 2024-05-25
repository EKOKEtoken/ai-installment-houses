import * as React from 'react';
import { useParams } from 'react-router-dom';
import Page from '../components/reusable/Page';

const Token = () => {
  const { tokenId } = useParams();

  return (
    <Page.BlankPage>
      <h1>Token {tokenId}</h1>
    </Page.BlankPage>
  );
};

export default Token;
