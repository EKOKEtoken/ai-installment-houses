import * as React from 'react';
import * as Icon from 'react-feather';

import Container from '../reusable/Container';
import { useAppContext } from './AppContext';

const AppSuccess = () => {
  const { appSuccess, setAppSuccess } = useAppContext();
  const [dismissTimeout, setDimissTimeout] = React.useState<NodeJS.Timeout>();

  const onDismiss = () => {
    setAppSuccess(undefined);
  };

  React.useEffect(() => {
    return () => {
      if (dismissTimeout) {
        clearTimeout(dismissTimeout);
      }
    };
  }, []);

  React.useEffect(() => {
    setDimissTimeout(
      setTimeout(() => {
        onDismiss();
      }, 5000),
    );
  }, [appSuccess]);

  if (appSuccess) {
    return (
      <Container.FlexRow className="top-4 fixed mx-auto border shadow-lg z-50 right-0 left-0 w-fit items-center gap-4 justify-between p-4 mb-4 text-sm text-green-800 rounded-lg bg-green-50 animate__animated animate__slideInDown">
        <Container.Container>
          <Icon.Info size={16} className="text-green-800 inline mr-2" />
          <span className="text-xs">{appSuccess}</span>
        </Container.Container>
        <Container.Container>
          <Icon.X size={16} className="text-green-800" onClick={onDismiss} />
        </Container.Container>
      </Container.FlexRow>
    );
  } else {
    return null;
  }
};

export default AppSuccess;
