import bodyParser from 'body-parser';
import cors from 'cors';
import express from 'express';
import multer from "multer";

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
// and convert async to AASX and save in ./Files/{id}/here.aasx
// res: folder structure
app.post("/api/set/file", async (req, res) => {
    const body = req.body;
    try {

    }
    catch (err) {

    }
});

// res: file contents (text / xml / picture / pdf) to view the file
app.get("/api/get/file", async (req, res) => {
    const body = req.body;
    const filePath = body.filePath;
    try {

    }
    catch (err) {

    }
});


// download converted file from path ./Files/{id}/file.aasx
// path is stored in table with id
// res: download-link for aasx file
app.get("/api/get/downloadAASX", async (req, res) => {
    const body = req.body;
    try {

    }
    catch (err) {

    }
});