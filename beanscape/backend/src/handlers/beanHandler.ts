import fs from 'fs';
import {Request, Response} from 'express';

const beanGetHandler = (req: Request, res: Response) => {
  if(req.params.name){
    var options = {
      root: 'public/beans',
      dotfiles: 'deny',
      headers: {
        'x-timestamp': Date.now(),
        'x-sent': true
      }
    }
    console.log(req.params?.name);
    res.sendFile(req.params.name, options,(err) => {
      if(err){
        if(err.name == "ENOENT"){
          res.sendStatus(404);
        } else {
          console.log(err);
          res.sendStatus(500);
        }
      }
    })
  }
  else{
    fs.readdir('public/beans',(err, files) => {
      if(err){
        console.log(err);
        res.send(500);
      } else {
        res.set('Cache-Control', 'no-cache, no-store, must-revalidate');
        res.json(files);
      }
    });
  }
}

const beanPostHandler = (req: Request, res: Response) => {
  if(!req.is('image/png')){
    res.sendStatus(400);
  } else {
    const filename = req.query.name;
    if(!filename){
      res.sendStatus(400);
    } else {
      const path = 'public/beans/' + filename + '.png';
      fs.writeFile(path,req.body,{flag:'wx'},(err) => {
        if(err){
          if(err.code == 'EEXIST'){
            res.sendStatus(409);
          } else {
            console.log(err);
            res.sendStatus(500);
          }
        } else {
          res.sendStatus(200);
        }
      });
    }
  }
}

const beanDeleteHandler = (req: Request, res: Response) => {
  const filename = req.params.name;
  const path = 'public/beans/' + filename;
  console.log(path);
  try {
    fs.unlinkSync(path);
    res.sendStatus(200);
  } catch(err){
    if (err) {
      if(err.code == "ENOENT"){
        res.sendStatus(404);
      } else {
        console.log(err);
        res.sendStatus(500);
      }
    }
  }
}

export {beanGetHandler, beanPostHandler, beanDeleteHandler};