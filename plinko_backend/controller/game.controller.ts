import { Request, Response } from 'express';
import { getPayload } from '../middleware/auth.middleware';
import { sendRequest } from './utils'; 
const settings = require('../settings.json');

const getLineIndex = (lines: number) => {
    let index = [8, 12, 16].indexOf(lines);
    return index === -1 ? 0 : index;
}

const random = () => {
    return Math.floor(Math.random() * 10000);
}

export const play = async (req: Request, res: Response) => {
    const payload = getPayload(req);
    if (!payload) return res.status(402).json('Unauthorized Wallet');

    const { wallet } = payload;
    console.log(wallet);

    const { lines, risk, tokenMint, betAmount, ballCount } = req.body;
    
    // Get tokenId Balance of wallet from DB
    const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
        endpoint: 'getPlayer',
        gameName: 'blinko',
        walletAddress: wallet,
        tokenSPLAddress: tokenMint
    }); 
    if (!player) {
        return res.status(500).json("There's no balance");
    }
    let balance = player.balance;

    let lineIndex = getLineIndex(lines);
    let maxBallCount = balance / betAmount;
    if (maxBallCount > ballCount) {
        maxBallCount = ballCount;
    }

    balance -= maxBallCount * betAmount;

    const result = [];
    const chance = [...settings.chance[risk][lineIndex]];
    const multiplier = settings.multiplier[risk][lineIndex];
    
    chance.forEach((percent: number, index: number) => {
        if (index) {
            chance[index] = chance[index - 1] + percent;
        }
    });

    for (let i = 0; i < maxBallCount; i++) {
        let target = lines / 2;
        for (let j = 0; j < lines; j++) {
            let rand = random();
            if (chance[j] > rand && rand > (j ? chance[j - 1] : 0)) {
                target = j;
                break;
            }
        }
        result[i] = target;
        balance = balance + multiplier[target] * betAmount;
    }

    // Update balance in the DB
    
    await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
        endpoint: 'updatePlayer',
        gameName: 'blinko',
        walletAddress: wallet,
        tokenSPLAddress: tokenMint,
        balance
    }); 
    return res.json(result);
}

export const getBalances = async (req: Request, res: Response) => {
    const payload = getPayload(req);
    if (!payload) return res.status(402).json('Unauthorized Wallet');

    const { wallet } = payload;
    console.log(wallet);

    // Get balances from DB
    const { data } = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
        endpoint: 'getPlayer',
        gameName: 'blinko',
        walletAddress: wallet
    });
    const balances = data.map((token: { tokenSPLAddress: any; balance: any; }) => {
        const { tokenSPLAddress, balance } = token;
        return { tokenSPLAddress, balance }
    });

    return res.json(balances);
}