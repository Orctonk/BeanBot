import createError, { HttpError } from 'http-errors';
import express, {Request, Response, ErrorRequestHandler, NextFunction} from 'express';
import path from 'path';
import cookieParser from 'cookie-parser';
import cors from 'cors';

import beanRouter from './routes/beans';

var app = express();

const PORT = process.env.PORT || '5000';

app.use(cors({
  origin: ['http://localhost:3000']
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
  console.log(`Listening on port ${PORT}`);
})
