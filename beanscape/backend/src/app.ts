import createError, { HttpError } from 'http-errors';
import express, {Request, Response, NextFunction} from 'express';
import cookieParser from 'cookie-parser';
import dotenv from 'dotenv';
import cors from 'cors';

import beanRouter from './routes/beans';
import { mkdirSync } from 'fs';

dotenv.config();

const PORT = process.env.PORT || '5000';

var app = express();

app.use(cors({
  origin: (process.env.CORS || '').split(',')
}));
app.use(express.urlencoded({ extended: false }));
app.use(express.json());
app.use(express.raw({
  type: 'image/png',
  limit: '10mb'
}));
app.use(cookieParser());

app.use('/beans', beanRouter);

// catch 404 and forward to error handler
app.use(function(req, res, next) {
  next(createError(404));
});

// error handler
app.use((err: HttpError ,req: Request, res: Response, next: NextFunction) => {
  // set locals, only providing error in development
  res.locals.message = err.message;
  res.locals.error = req.app.get('env') === 'development' ? err : {};

  // render the error page
  res.status(err.status || 500).send();
});


app.listen(PORT,() => {
  mkdirSync('public/beans', { recursive: true });
  console.log(`Listening on port ${PORT}`);
})
