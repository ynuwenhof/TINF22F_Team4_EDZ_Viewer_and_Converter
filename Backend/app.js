import bodyParser from 'body-parser';
import cors from 'cors';
import express from 'express';

const app = express();
app.use(bodyParser.json())
app.use(cors())
const port = process.env.PORT;

app.post("/api/set/file", async (req, res) => {
    const body = req.body;
    try {

    }
    catch (e) {

    }
});

app.get("/api/get/file", async (req, res) => {
    const body = req.body;
    try {

    }
    catch (e) {

    }
});