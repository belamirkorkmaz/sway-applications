import { Flex } from "@fuel-ui/react";

import { AssetOutput } from "~/systems/Core";
import type { NFTAssetOutput, TokenAssetOutput } from "~/types/contracts/AuctionContractAbi";

interface AuctionAssetInfoProps {
  sellAsset: NFTAssetOutput | TokenAssetOutput;
  sellAssetAmount: string;
  isSellAssetNFT: boolean;
  bidAsset: NFTAssetOutput | TokenAssetOutput;
  bidAssetAmount: string;
  isBidAssetNFT: boolean;
  initialPrice: string;
}

export const AuctionAssetInfo = ({
    sellAsset,
    sellAssetAmount,
    isSellAssetNFT,
    bidAsset,
    bidAssetAmount,
    isBidAssetNFT,
    initialPrice,
}: AuctionAssetInfoProps) => {
  return (
    <Flex>
      <AssetOutput
        assetId={sellAsset.asset_id.value}
        assetAmount={sellAssetAmount}
        heading="Selling"
        isNFT={isSellAssetNFT}
      />

      <AssetOutput
        assetId={bidAsset.asset_id.value}
        assetAmount={bidAssetAmount}
        heading="Highest Bid"
        isNFT={isBidAssetNFT}
      />

      <AssetOutput
        assetId={bidAsset.asset_id.value}
        assetAmount={initialPrice}
        heading="Initial Price"
        isNFT={isBidAssetNFT}
      />
    </Flex>
  );
};
