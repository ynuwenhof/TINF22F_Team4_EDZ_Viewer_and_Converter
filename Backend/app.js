import bodyParser from 'body-parser';
import cors from 'cors';
import express from 'express';
import multer from "multer";
import {convert} from "./file";
import {verifyToken} from "./token";

const app = express();
app.use(bodyParser.json())
app.use(cors())
const port = process.env.PORT;

// Configuration of Multer, setting saving path of upload
const storage = multer.diskStorage({
    destination: function (req, file, cb) {
        cb(null, 'uploads/') // path
    },
    filename: function (req, file, cb) {
        cb(null, file.fieldname + '-' + Date.now())
    }
});

const upload = multer({ storage: storage });

// first edz-file upload
// insert to table in SQLite an entry with path to file
// should save the edz file in ../Files/{id}/here.edz (important if folder exists, rename folder)
// generate with returned id a token and add id to response
// res: folder structure
app.post("/api/set/file", async (req, res) => {
    const body = req.body;
    try {

    }
    catch (err) {
        res.status(400).json({ success: false, message: err });
    }
});

// convert to AASX and save in ./Files/{id}/here.aasx
app.get("/api/get/convert", async (req, res) => {
    const token = req.headers.authorization.split(' ')[1];
    try {
        const id = verifyToken(token)

    }
    catch (err) {
        res.status(400).json({ success: false, message: err });
    }
})

// res: file contents (text / xml / picture / pdf) to view the file
app.get("/api/get/file", async (req, res) => {
    const token = req.headers.authorization.split(' ')[1];
    const body = req.body;
    const filePath = body.filePath;
    try {
        const id = verifyToken(token)

    }
    catch (err) {
        res.status(400).json({ success: false, message: err });
    }
});


// download converted file from path ./Files/{id}/file.aasx
// path is stored in table with id
// res: download-link for aasx file
app.get("/api/get/downloadAASX", async (req, res) => {
    const token = req.headers.authorization.split(' ')[1];
    const body = req.body;
    try {
        const id = verifyToken(token)

    }
    catch (err) {
        res.status(400).json({ success: false, message: err });
    }
});


// host server
app.listen(port, () => {
    console.log(`Server is running on https://localhost:${port}`);
});