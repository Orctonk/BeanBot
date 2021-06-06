import express from 'express';
import {beanGetHandler, beanPostHandler, beanDeleteHandler} from '../handlers/beanHandler';
import jwt from 'express-jwt';
import jwks from 'jwks-rsa';

var router = express.Router();

var jwtCheck = jwt({
  secret: jwks.expressJwtSecret({
    cache: true,
    rateLimit: true,
    jwksRequestsPerMinute: 5,
    jwksUri: 'https://beanscape.eu.auth0.com/.well-known/jwks.json'
  }),
  audience: 'https://api.beanscape.dev',
  issuer: 'https://beanscape.eu.auth0.com/',
  algorithms: ['RS256']
});

router.get('/', beanGetHandler);
router.get('/:name', beanGetHandler);
router.post('/', jwtCheck, beanPostHandler);
router.delete('/:name', jwtCheck, beanDeleteHandler);

export default router;
