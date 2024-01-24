import { Request, Response } from "express"
import fs from 'fs';

const settings = require("../settings.json");

export const getSettings = (req: Request, res: Response) => {
    return res.json({ multiplier: settings.multiplier });
}

export const getAdminSettings = (req: Request, res: Response) => {
    return res.json(settings);
}

export const setSettings = (req: Request, res: Response) => {
    const { multiplier, chance } = req.body;
    const settingsContent = JSON.stringify({ multiplier, chance });
    fs.writeFile("./settings.json", settingsContent, (err) => {
        if (err) {
            console.log(err);
            return res.status(500).json("Failed to save settings");
        }
        console.log("Settings file has been saved.");
        return res.json("Success");
    });
}

export const setNonceAccount = (req: Request, res: Response) => {
    const { nonceAccount } = req.body;
    const configContent = JSON.stringify({ nonceAccount });
    fs.writeFile("./config.json", configContent, (err) => {
        if (err) {
            console.log(err);
            return res.status(500).json("Failed to save nonce account");
        }
        console.log("Config file has been saved")
        return res.json("Success");
    });
}

export const getNonceAccount = (req: Request, res: Response) => {
    const { nonceAccount } = require('../config.json');
    return res.json(nonceAccount);
}