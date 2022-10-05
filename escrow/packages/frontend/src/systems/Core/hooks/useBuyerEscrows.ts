import { useQuery } from 'react-query';
import { bn } from 'fuels';

import { useWallet } from './useWallet';

import { useContract } from './useContract';
import { useEscrows } from './useEscrows';

export function useBuyerEscrows() {
  const contract = useContract();
  const wallet = useWallet();

  const { data: buyerEscrowIds } = useQuery(
    ['BuyerPage-buyerEscrowIds', wallet?.address.toHexString()],
    async () => {
      return (
        contract &&
        wallet &&
        (
          await contract!.functions
            .buyer_escrows({ Address: { value: wallet?.address!.toHexString() } })
            .get()
        ).value
      );
    }
  );

  const buyerEscrows = useEscrows('BuyerEscrows', buyerEscrowIds);

  return { buyerEscrows, buyerEscrowIds: !buyerEscrowIds ? [bn(0)] : buyerEscrowIds };
}
