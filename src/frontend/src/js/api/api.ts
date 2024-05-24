/**
 * @description Sends a request to the ledger API
 * @param method method name to call
 * @param mock a mock to return in case of no API URL
 * @param params the parameters to send to the API
 * @param onError a callback to call in case of an error
 * @returns
 */
export const ledgerJsonRequest = async <T>(
  method: string,
  mock: T,
  params?: any,
  onError?: (statusCode: number, error: any) => void,
): Promise<T> => {
  const apiUrl = process.env.LEDGER_API_URL;
  if (!apiUrl) {
    console.log('MOCK', method, params);
    return mock;
  }

  return await sendJsonRequest(apiUrl, method, params, onError);
};

/**
 * @description Sends a request to the swap API
 * @param method method name to call
 * @param mock a mock to return in case of no API URL
 * @param params the parameters to send to the API
 * @param onError a callback to call in case of an error
 * @returns
 */
export const swapJsonRequest = async <T>(
  method: string,
  mock: T,
  params?: any,
  onError?: (statusCode: number, error: any) => void,
): Promise<T> => {
  const apiUrl = process.env.SWAP_API_URL;
  if (!apiUrl) {
    console.log('MOCK', method, params);
    return mock;
  }

  return await sendJsonRequest(apiUrl, method, params, onError);
};

const sendJsonRequest = async <T>(
  url: string,
  method: string,
  params?: any,
  onError?: (statusCode: number, error: any) => void,
): Promise<T> => {
  const headers = {
    'Content-Type': 'application/json',
  };

  const body = params
    ? {
        method,
        params,
      }
    : { method };

  const response = await fetch(url, {
    body: JSON.stringify(body),
    method: 'GET',
    headers,
    credentials: 'include',
  });

  if (response.ok) {
    try {
      return await response.json();
    } catch (e) {
      console.log(
        'Error parsing JSON',
        e,
        response.status,
        response.statusText,
        response.url,
        response.body,
      );
      return {} as T;
    }
  }

  const statusCode = response.status;
  const error = await response.text();

  if (onError) {
    onError(statusCode, error);
  }

  throw new Error(error);
};

export const makeQueryArgs = <T extends object>(opts: T): string => {
  return Object.keys(opts)
    .filter((key) => {
      const value = opts[key as keyof T];

      if (value === undefined) return false;

      if (typeof value === 'boolean' && value === false) return false;

      return true;
    })
    .map((key) => {
      const value = opts[key as keyof T];
      if (value === undefined) {
        return '';
      }

      if (typeof value === 'boolean' && value) {
        return key;
      }

      if (value === null) {
        return `${key}`;
      }

      if (
        typeof value === 'string' ||
        typeof value === 'number' ||
        typeof value === 'boolean'
      ) {
        return `${key}=${encodeURIComponent(value)}`;
      }

      return '';
    })
    .join('&');
};
