"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const beanHandler_1 = require("../handlers/beanHandler");
const express_jwt_1 = __importDefault(require("express-jwt"));
const jwks_rsa_1 = __importDefault(require("jwks-rsa"));
var router = express_1.default.Router();
var jwtCheck = express_jwt_1.default({
    secret: jwks_rsa_1.default.expressJwtSecret({
        cache: true,
        rateLimit: true,
        jwksRequestsPerMinute: 5,
        jwksUri: 'https://beanscape.eu.auth0.com/.well-known/jwks.json'
    }),
    audience: 'https://api.beanscape.dev',
    issuer: 'https://beanscape.eu.auth0.com/',
    algorithms: ['RS256']
});
router.get('/', beanHandler_1.beanGetHandler);
router.get('/:name', beanHandler_1.beanGetHandler);
router.post('/', jwtCheck, beanHandler_1.beanPostHandler);
router.delete('/:name', jwtCheck, beanHandler_1.beanDeleteHandler);
exports.default = router;
