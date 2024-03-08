import jwt from 'jsonwebtoken';
import dotenv from 'dotenv'
import {existsFile} from './file.js';

dotenv.config()


const secretKey = process.env.TOKEN_KEY;


export async function generateToken(fileID) {
    return jwt.sign({fileID: fileID}, secretKey, {expiresIn: '20 min'});
}

export async function verifyToken(token) {
    try {
        const decoded = jwt.verify(token, secretKey);
        existsFile(decoded.fileID);
        return decoded;
    } catch (err) {
        throw new Error('The token has expired or is invalid. Please upload file again. : ' + err.message)
    }
}