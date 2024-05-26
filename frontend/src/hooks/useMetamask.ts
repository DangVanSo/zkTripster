import { useState, useEffect } from 'react';
import {createPublicClient,  createWalletClient, custom} from 'viem';
import { sepolia } from 'viem/chains';
import { MetaMaskInpageProvider } from "@metamask/providers";

declare global {
    interface Window{
        ethereum?:MetaMaskInpageProvider
    }
}

const useMetaMask = () => {
    const [isConnected, setIsConnected] = useState(false);
    const [account, setAccount] = useState(null);
    const [walletClient, setWalletClient] = useState(null);
    const [client, setClient] = useState(null);

    const connectMetaMask = async () => {
        if (window.ethereum) {
            try {
                await window.ethereum.request({ method: 'eth_requestAccounts' });
                const walletClient = createWalletClient({
                    transport: custom(window.ethereum),
                    chain: sepolia,
                });

                const accounts = await walletClient.request({ method: 'eth_accounts' }) as string[];
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                setWalletClient(walletClient)
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                setAccount(accounts[0]);
                setIsConnected(true);
            } catch (error) {
                console.error("User denied account access or there was an error", error);
            }
        } else {
            console.error("MetaMask is not installed");
        }
    };

    useEffect(() => {
        const checkConnection = async () => {
            if (window.ethereum) {
                const publicClient = createPublicClient({
                    chain: sepolia,
                    transport: custom(window.ethereum),
                });

                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                const accounts = await publicClient.request({ method: 'eth_accounts' }) as string[];
                if (accounts.length > 0) {
                    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                    // @ts-expect-error
                    setClient(publicClient);
                    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                    // @ts-expect-error
                    setAccount(accounts[0]);
                    setIsConnected(true);
                }
            }
        };
        checkConnection();
    }, []);

    return { isConnected, account, connectMetaMask, walletClient, client };
};

export default useMetaMask;
