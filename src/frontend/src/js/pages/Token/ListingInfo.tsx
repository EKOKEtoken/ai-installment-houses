import * as React from 'react';
import * as Icon from 'react-feather';
import { Principal } from '@dfinity/principal';
import { useIcWallet } from 'react-ic-wallet';

import RealEstate from '../../data/real_estate';
import { useAppContext } from '../../components/App/AppContext';
import Container from '../../components/reusable/Container';
import icpToUsd from '../../api/icpToUsd';
import { ApiListing } from '../../data/api';
import getListing from '../../api/getListing';
import { e8sToIcpNumber } from '../../utils';
import IcConnect from '../../components/IcConnect';
import Button from '../../components/reusable/Button';
import Link from '../../components/reusable/Link';
import { Route } from '../../utils/routes';
import { useAgentContext } from '../../ic/AgentContext';
import TaskList from '../../components/shared/TaskList';
import buyTokenTasks from '../../tasks/BuyTokenTask';

interface Props {
  realEstate: RealEstate;
}

const ListingInfo = ({ realEstate }: Props) => {
  const { setAppError, setAppSuccess } = useAppContext();
  const { icpLedger, swap } = useAgentContext();
  const { status, principal } = useIcWallet();
  const [listing, setListing] = React.useState<ApiListing>();
  const [usdPrice, setUsdPrice] = React.useState<number | null>();
  const [buyToken, setBuyToken] = React.useState<boolean>(false);
  const [tokenBought, setTokenBought] = React.useState<boolean>(false);

  const onBuyToken = () => {
    if (icpLedger === undefined || swap === undefined) {
      setAppError('Ledger or swap canisters not available');
      return;
    }
    setBuyToken(true);
  };

  const onTokenBought = () => {
    setBuyToken(false);
    setTokenBought(true);
    setAppSuccess('Token bought successfully');
  };

  React.useEffect(() => {
    if (listing !== undefined) {
      icpToUsd(e8sToIcpNumber(listing.icpPrice))
        .then((price) => {
          setUsdPrice(price);
        })
        .catch((e) => {
          console.error(e);
          setAppError("Couldn't fetch USD price");
        });
    }
  }, [listing]);

  React.useEffect(() => {
    getListing(realEstate.id)
      .then((data) => {
        if (data !== null) {
          setListing(data.listing);
        }
      })
      .catch((e) => {
        console.error(e);
        setAppError('Failed to fetch token listing');
      });
  }, [realEstate]);

  if (listing === undefined) {
    return null;
  }

  const icpPrice = e8sToIcpNumber(listing.icpPrice);
  const expirationMs = Number(listing.expirationNs / BigInt(1_000_000));
  const expiration = new Date(expirationMs);

  return (
    <>
      <Container.Container className="border border-gray-200 rounded p-4">
        <Container.FlexCols className="gap-1">
          <Container.Container>
            <span className="block text-text">Current price</span>
          </Container.Container>
          <Container.FlexRow className="gap-4 items-center">
            <span className="block text-brand text-xl">
              {icpPrice.toLocaleString('en-US', {
                style: 'currency',
                currency: 'ICP',
                minimumFractionDigits: 2,
                maximumFractionDigits: 8,
              })}
            </span>
            {usdPrice != null && (
              <span className="block text-text text-sm">
                {usdPrice.toLocaleString('en-US', {
                  style: 'currency',
                  currency: 'USD',
                  minimumFractionDigits: 2,
                  maximumFractionDigits: 2,
                })}
              </span>
            )}
          </Container.FlexRow>
          <Container.Container>
            <span className="block text-text text-sm">
              Listed until: {expiration.toLocaleDateString('en-US')}
            </span>
          </Container.Container>
          <Container.Container className="pt-2">
            {CtaDispatch(
              realEstate,
              status,
              principal,
              tokenBought,
              onBuyToken,
            )}
          </Container.Container>
        </Container.FlexCols>
      </Container.Container>
      {swap === undefined || icpLedger === undefined ? null : (
        <TaskList
          onDone={onTokenBought}
          run={buyToken}
          title={`Buying token #${realEstate.id}`}
          tasks={buyTokenTasks(realEstate, listing, swap, icpLedger)}
        />
      )}
    </>
  );
};

const CtaDispatch = (
  token: RealEstate,
  icStatus: string,
  principal: Principal | null,
  tokenBought: boolean,
  onBuyToken: () => void,
): React.ReactNode => {
  if (icStatus !== 'connected') {
    return <IcConnect />;
  }

  const principalStr = principal?.toText();

  // if same owner, edit listing page
  if (principalStr === token.owner || tokenBought) {
    return (
      <Link.Button href={Route.editListingUrl(token.id)}>
        Edit listing <Icon.ArrowRight className="inline ml-2" size={20} />
      </Link.Button>
    );
  }

  return (
    <Button.Primary onClick={onBuyToken}>
      <Icon.ShoppingCart className="inline mr-2" size={20} />
      Buy now
    </Button.Primary>
  );
};

export default ListingInfo;
