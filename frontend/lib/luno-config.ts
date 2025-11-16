import { createConfig } from "@luno-kit/react";
import { kusama, polkadot } from "@luno-kit/react/chains";
import {
  talismanConnector,
  subwalletConnector,
  polkadotjsConnector,
} from "@luno-kit/react/connectors";

export const lunoConfig = createConfig({
  appName: "Dot.Repute - Rust-Powered Contributor Reputation System",
  chains: [polkadot, kusama],
  connectors: [
    talismanConnector(),
    subwalletConnector(),
    polkadotjsConnector(),
  ],
  autoConnect: true,
});
