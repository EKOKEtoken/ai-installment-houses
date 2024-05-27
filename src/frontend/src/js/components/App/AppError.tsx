import * as React from 'react';
import * as Icon from 'react-feather';

import { useAppContext } from './AppContext';
import Container from '../reusable/Container';

const AppError = () => {
  const { appError, setAppError } = useAppContext();

  const onDismiss = () => {
    setAppError(undefined);
  };

  if (appError) {
    return (
      <Container.FlexRow className="top-4 fixed mx-auto border shadow-lg z-50 right-0 left-0 w-fit items-center gap-4 justify-between p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 animate__animated animate__slideInDown">
        <Container.Container>
          <Icon.Info size={16} className="text-red-800 inline mr-2" />
          <span className="text-xs">{appError}</span>
        </Container.Container>
        <Container.Container>
          <Icon.X size={16} className="text-red-800" onClick={onDismiss} />
        </Container.Container>
      </Container.FlexRow>
    );
  } else {
    return null;
  }
};

export default AppError;
