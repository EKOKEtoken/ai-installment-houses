import * as React from 'react';
import { ActorSubclass } from '@dfinity/agent';
import { useIcWallet } from 'react-ic-wallet';

import { IcpLedger, idlFactory as icpLedgerIdlFactory } from './icp-ledger.did';
import { Ledger, idlFactory as ledgerIdlFactory } from './ledger.did';
import { Swap, idlFactory as swapIdlFactory } from './swap.did';

export const ICP_LEDGER_CANISTER_ID = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
export const LEDGER_CANISTER_ID = '';
export const SWAP_CANISTER_ID = '';

interface Context {
  icpLedger?: ActorSubclass<IcpLedger>;
  ledger?: ActorSubclass<Ledger>;
  swap?: ActorSubclass<Swap>;
}

const AgentContext = React.createContext<Context>({});

const AgentContextProvider = ({ children }: { children?: React.ReactNode }) => {
  const [icpLedger, setIcpLedger] = React.useState<ActorSubclass<IcpLedger>>();
  const [ledger, setLedger] = React.useState<ActorSubclass<Ledger>>();
  const [swap, setSwap] = React.useState<ActorSubclass<Swap>>();

  const { createActor, status } = useIcWallet();

  React.useEffect(() => {
    if (status === 'connected') {
      createActor(ICP_LEDGER_CANISTER_ID, icpLedgerIdlFactory).then((actor) => {
        setIcpLedger(actor as ActorSubclass<IcpLedger>);
      });
      createActor(LEDGER_CANISTER_ID, ledgerIdlFactory).then((actor) => {
        setLedger(actor as ActorSubclass<Ledger>);
      });
      createActor(SWAP_CANISTER_ID, swapIdlFactory).then((actor) => {
        setSwap(actor as ActorSubclass<Swap>);
      });
    } else {
      setLedger(undefined);
      setSwap(undefined);
    }
  }, [status]);

  return (
    <AgentContext.Provider value={{ icpLedger, ledger, swap }}>
      {children}
    </AgentContext.Provider>
  );
};

export default AgentContextProvider;

export const useAgentContext = () => React.useContext(AgentContext);
