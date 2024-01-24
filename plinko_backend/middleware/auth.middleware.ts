import { PublicKey } from '@solana/web3.js';
import { Request, Response, NextFunction } from 'express';
import nacl from 'tweetnacl';
import { authorizedAdmins } from '../constants';
import jwt from 'jsonwebtoken';

type Payload = {
    wallet: string;
    message: string;
    signature: number[];
}

export function getPayload(req: Request): Payload | undefined {
    const authorizationHeader = req.headers['authorization'];
    
    let token;
    if (authorizationHeader) {
        token = authorizationHeader.split(' ')[1];
    }
    if (token) {
        const { payload } = jwt.verify(token, process.env.TOKEN_SECRET_KEY || '', { complete: true });
        return payload as Payload;
    }
}

function isVerified(payload: Payload) {
    const { message, wallet, signature } = payload;
    const verified = nacl.sign.detached.verify(
        new Uint8Array(Buffer.from(message)), 
        new Uint8Array(Buffer.from(signature)), 
        new PublicKey(wallet).toBytes()
    );
    return verified;
}

export function authorizedPlayer(req: Request, res: Response, next: NextFunction) {
    const payload = getPayload(req);
    if (payload) {
        if (isVerified(payload)) {
            next();
            return;
        }
    }
    return res.status(402).json("Unauthorized Wallet");
}

export function authorizedAdmin(req: Request, res: Response, next: NextFunction) {
    const payload = getPayload(req);
    if (payload) {
        const { wallet } = payload;
        if (isVerified(payload) && authorizedAdmins.includes(wallet)) {
            next();
            return;
        }
    }
    return res.status(402).json("Unauthorized Wallet");
}