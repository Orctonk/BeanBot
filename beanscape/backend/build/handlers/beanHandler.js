"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.beanDeleteHandler = exports.beanPostHandler = exports.beanGetHandler = void 0;
const fs_1 = __importDefault(require("fs"));
const beanGetHandler = (req, res) => {
    var _a;
    if (req.params.name) {
        var options = {
            root: 'public/beans',
            dotfiles: 'deny',
            headers: {
                'x-timestamp': Date.now(),
                'x-sent': true
            }
        };
        console.log((_a = req.params) === null || _a === void 0 ? void 0 : _a.name);
        res.sendFile(req.params.name, options, (err) => {
            if (err) {
                if (err.name == "ENOENT") {
                    res.sendStatus(404);
                }
                else {
                    console.log(err);
                    res.sendStatus(500);
                }
            }
        });
    }
    else {
        fs_1.default.readdir('public/beans', (err, files) => {
            if (err) {
                console.log(err);
                res.send(500);
            }
            else {
                res.set('Cache-Control', 'no-cache, no-store, must-revalidate');
                res.json(files);
            }
        });
    }
};
exports.beanGetHandler = beanGetHandler;
const beanPostHandler = (req, res) => {
    if (!req.is('image/png')) {
        res.sendStatus(400);
    }
    else {
        const filename = req.query.name;
        if (!filename) {
            res.sendStatus(400);
        }
        else {
            const path = 'public/beans/' + filename + '.png';
            fs_1.default.writeFile(path, req.body, { flag: 'wx' }, (err) => {
                if (err) {
                    if (err.code == 'EEXIST') {
                        res.sendStatus(409);
                    }
                    else {
                        console.log(err);
                        res.sendStatus(500);
                    }
                }
                else {
                    res.sendStatus(200);
                }
            });
        }
    }
};
exports.beanPostHandler = beanPostHandler;
const beanDeleteHandler = (req, res) => {
    const filename = req.params.name;
    const path = 'public/beans/' + filename;
    console.log(path);
    try {
        fs_1.default.unlinkSync(path);
        res.sendStatus(200);
    }
    catch (err) {
        if (err) {
            if (err.code == "ENOENT") {
                res.sendStatus(404);
            }
            else {
                console.log(err);
                res.sendStatus(500);
            }
        }
    }
};
exports.beanDeleteHandler = beanDeleteHandler;
