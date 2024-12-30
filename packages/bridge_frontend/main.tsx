import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import {
  InternetIdentityProvider,
  useInternetIdentity,
} from 'ic-use-internet-identity';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Actors from './actor.tsx';
import { createRouter, RouterProvider } from '@tanstack/react-router';
import { routeTree } from './routeTree.gen.ts';
import { SiweIdentityProvider } from 'ic-use-siwe-identity';
import { _SERVICE } from '../ic_siwe_provider/declarations/ic_siwe_provider.did';
import { canisterId, idlFactory } from '../ic_siwe_provider/declarations/index';
import { wagmiConfig } from './wagmi/wagmi.config.ts';
import { WagmiProvider } from 'wagmi';

export const router = createRouter({
  routeTree,
  context: {
    identity: undefined!,
  },
});

// Mimimize reloading of queries
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retryOnMount: false,
      retry: false,
      gcTime: Infinity,
      staleTime: Infinity,
    },
  },
});

function InnerRoot() {
  const { identity, isInitializing } = useInternetIdentity();

  if (isInitializing) return null;

  return <RouterProvider router={router} context={{ identity }} />;
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <InternetIdentityProvider>
          <SiweIdentityProvider<_SERVICE>
            canisterId={canisterId}
            idlFactory={idlFactory}
          >
            <Actors>
              <InnerRoot />
            </Actors>
          </SiweIdentityProvider>
        </InternetIdentityProvider>
      </QueryClientProvider>
    </WagmiProvider>
  </StrictMode>,
);
