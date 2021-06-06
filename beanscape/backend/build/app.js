"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const http_errors_1 = __importDefault(require("http-errors"));
const express_1 = __importDefault(require("express"));
const cookie_parser_1 = __importDefault(require("cookie-parser"));
const cors_1 = __importDefault(require("cors"));
const beans_1 = __importDefault(require("./routes/beans"));
var app = express_1.default();
const PORT = process.env.PORT || '5000';
app.use(cors_1.default({
    origin: ['http://localhost:3000']
}));
app.use(express_1.default.urlencoded({ extended: false }));
app.use(express_1.default.json());
app.use(express_1.default.raw({
    type: 'image/png',
    limit: '10mb'
}));
app.use(cookie_parser_1.default());
app.use('/beans', beans_1.default);
// catch 404 and forward to error handler
app.use(function (req, res, next) {
    next(http_errors_1.default(404));
});
// error handler
app.use((err, req, res, next) => {
    // set locals, only providing error in development
    res.locals.message = err.message;
    res.locals.error = req.app.get('env') === 'development' ? err : {};
    // render the error page
    res.status(err.status || 500).send();
});
app.listen(PORT, () => {
    console.log(`Listening on port ${PORT}`);
});
