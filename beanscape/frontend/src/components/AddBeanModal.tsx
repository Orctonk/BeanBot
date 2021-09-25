import React, { useState } from 'react';
import { Box, Button, Modal, TextField, Typography } from '@material-ui/core';
import useStyle from '../styles/GlobalStyle';

export interface Props {
  open: boolean,
  onSave: (name:string, file:File) => void,
  onClose: () => void
};

export const prettyBeanName = (beanfile: string | undefined) => {
  if (!beanfile){
    return undefined;
  }
  const lastdot = beanfile.lastIndexOf('.');
  let name = beanfile.substring(0,lastdot);

  name = name[0].toUpperCase() + name.substring(1);
  name = name.replaceAll(/^_*|_*$/g,'');
  name = name.replaceAll(/_\w?/g, (value) => ' ' + value[1].toUpperCase());
  return name;
}

export const beanFileName = (beanName: string) => {
  let name = beanName;
  name = name.replaceAll(/ /g,'_');
  name = name.replaceAll(/[A-Z]/g,(value) => value.toLowerCase());
  return name;
}

export const AddBeanModal = ({open, onSave, onClose}: Props) => {
  const classes = useStyle();
  const [name, setName] = useState<string>('');
  const [nameError, setNameError] = useState<boolean>(false);
  const [fileError, setFileError] = useState<boolean>(false);
  const [file, setFile] = useState<File>();

  return (
    <Modal open={open}>
      <Box>
        <Box className={classes.modalBox}>
          <Typography variant="h5">Add a new bean</Typography>
          <TextField 
            label="Bean name"
            onChange={(value) => setName(value.currentTarget.value)}
            value={name}
            error={nameError}
            required
          />
          {
            nameError && 
            <Typography variant="caption" color="error">Please enter a bean name</Typography>
          }
          
          <Button
            variant='contained'
            style={{marginTop: '10px'}}
            component='label'>
            <input type="file" 
              onChange={(value) => {
                const newFile = (value.currentTarget.files ?? [])[0];
                setFile(newFile);
                if(!name){
                  setName(prettyBeanName(newFile?.name) ?? '');
                }
              } }
              hidden 
              required/>
            {
              file ? file.name:
              "Select a file"
            }
          </Button>
          {
            fileError && 
            <Typography variant="caption" color="error">Please select a bean to upload</Typography>
          }
          <Box>
            <Button className={classes.modalSave} onClick={() => {
              setNameError(false);
              setFileError(false);
              if(!name){
                console.log(`nameError`);
                setNameError(true);
              }
              if(!file){
                console.log(`fileError`);
                setFileError(true);
              }
              if(!fileError && !nameError) {
                console.log(`${fileError}, ${nameError}`);
                onSave(name,file as File);
                onClose();
              }
            }}>Save</Button>
            <Button className={classes.modalCancel} onClick={onClose}>Close</Button>
          </Box>
        </Box>
      </Box>
    </Modal>
  );
}