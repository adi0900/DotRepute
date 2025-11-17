import { checkAddress } from "@polkadot/util-crypto";

const SUPPORTED_NETWORKS = [
  { prefix: 0, name: "Polkadot", icon: "Polkadot" },
  { prefix: 2, name: "Kusama", icon: "Kusama" },
];

export function extractPolkadotAddress(
  message: string
): { address: string; network: string } | null {
  const words = message.trim().split(/\s+/);

  for (const word of words) {
    if (word.length < 47 || word.length > 50) continue;

    if (!/^[1-9A-HJ-NP-Za-km-z]+$/.test(word)) continue;

    for (const net of SUPPORTED_NETWORKS) {
      const [isValid] = checkAddress(word, net.prefix);
      if (isValid) {
        return { address: word, network: net.name };
      }
    }
  }

  return null;
}
