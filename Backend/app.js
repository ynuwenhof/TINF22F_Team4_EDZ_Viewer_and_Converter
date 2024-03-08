import bodyParser from 'body-parser';
import fs from 'fs';
import cors from 'cors';
import express from 'express';

const options = {
    key: fs.readFileSync('./server.key'),
    cert: fs.readFileSync('./server.cert')
};

const app = express();
app.use(bodyParser.json())
app.use(cors())
const port = process.env.PORT;