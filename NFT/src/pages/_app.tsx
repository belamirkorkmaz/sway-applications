import "@/styles/globals.css";
import type { NextPage } from "next";
import type { AppProps } from "next/app";
import React from "react";
import Head from "next/head";
import { HydrationBoundary } from "@tanstack/react-query";
import { AppProvider } from "@/components/Provider";
import { useRouter } from "next/router";
import { getNFTLayout } from "@/utils/getNFTLayout";

export type NextPageWithLayout = NextPage & {
  getLayout?: (page: React.ReactElement) => React.ReactNode;
};

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

export default function App({ Component, pageProps }: AppPropsWithLayout) {
  const router = useRouter();
  // NOTE: only apply the nft app layout to the nft app
  const getLayout = router.route.includes("nft")
    ? getNFTLayout
    : (page: React.ReactElement) => {
        return <>{page}</>;
      };

  return (
    <AppProvider>
      {/** https://tanstack.com/query/latest/docs/framework/react/guides/ssr */}
      <HydrationBoundary state={pageProps.dehydratedState}>
        <Head>
          <title>Fuel App</title>
          <link rel="icon" href="/fuel.ico" />
        </Head>

        {getLayout(<Component {...pageProps} />)}
      </HydrationBoundary>
    </AppProvider>
  );
}
