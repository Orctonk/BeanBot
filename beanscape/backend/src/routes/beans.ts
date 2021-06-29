import express from 'express';
import {beanGetHandler, beanPostHandler, beanDeleteHandler} from '../handlers/beanHandler';
import jwt from 'express-jwt';
import jwks from 'jwks-rsa';
import jwtAuthz from 'express-jwt-authz';

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

const checkAdminScope = jwtAuthz([ 'write:beans' ]);

router.get('/', beanGetHandler);
router.get('/:name', beanGetHandler);
router.post('/', jwtCheck, checkAdminScope, beanPostHandler);
router.delete('/:name', jwtCheck, checkAdminScope, beanDeleteHandler);

export default router;
