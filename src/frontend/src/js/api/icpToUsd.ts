const icpToUsd = async (icp: number) => {
  const response = await fetch(
    `https://api.coinconvert.net/convert/icp/usd?amount=${icp}`,
    {
      method: 'GET',
    },
  );

  if (response.ok) {
    const json = await response.json();
    return json['USD'];
  }

  const error = await response.text();
  throw new Error(error);
};

export default icpToUsd;
