import { KmsSigner } from "./src/kms-signer";
import { BluefinProSdk, BluefinRequestSigner } from "./src";
import { SuiClient } from "@firefly-exchange/library-sui";


(async () => {
    for (const str of [
        "alias/key-alias-1",
    ]) {
        const kms = new KmsSigner(str, {
            region: "ap-northeast-1",
            accessKeyId: "",
            secretAccessKey: "",
            sessionToken: "",
        });
        await kms.init();

        const client = new SuiClient({
            url: "https://fullnode.testnet.sui.io:443",
        });
        const wallet =
            Object.assign(kms, {
                isUIWallet: () => false,
            });
        const bluefinSigner = new BluefinRequestSigner(wallet);
        const sdk = new BluefinProSdk(bluefinSigner, "devnet", client, {});

        await sdk.initialize();
        const address = kms.getPublicKey().toSuiAddress();
        try {
            const account = await sdk.accountDataApi.getAccountDetails(address);
            const amount = account.data.assets[0]?.quantityE9;
            if (amount && Number(amount) > 0) {
                const amountToWithdraw = Number(amount) - 100 * 1e9;
                const amountToDeposit = 10000 * 1e9;
                console.log(address, amount, amountToWithdraw, amountToDeposit);
                try {
                    (<any>global).useKmsV2 = true;
                    const tx = await sdk.withdraw("USDC", amountToWithdraw.toString());
                    // const tx2 = await sdk.deposit("USDC", amountToDeposit.toString());
                    console.log(tx);
                    // console.log(tx2);
                } catch (e) {
                    console.log((e as Error).message);
                    console.log((e as any).response?.message);
                } finally {
                    (<any>global).useKmsV2 = false;
                }
            }
        } catch (e) {
            console.log(e);
        }
    }
})();
