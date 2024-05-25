import * as crypto from 'crypto';

var ecdh = crypto.createECDH('secp256k1');
import { Chacha20 } from "ts-chacha20";

export function decryptEcdhChacha20(cipher: Buffer, sellerPk: Uint8Array, buyerSk: Uint8Array, nonce: Uint8Array): Uint8Array {
    ecdh.setPrivateKey(buyerSk);
    const sharedKey = ecdh.computeSecret(sellerPk);
    return new Chacha20(sharedKey, nonce).decrypt(cipher);
}
