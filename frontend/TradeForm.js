import React, { useState } from 'react';
import axios from 'axios';

const TradeDataUpload = () => {
  const [file, setFile] = useState(null);

  const handleFileChange = (event) => {
    setFile(event.target.files[0]);
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    if (!file) {
      handleNoFileSelected();
      return;
    }
    await uploadFile(file);
  };

  const handleNoFileSelected = () => {
    alert('Please select a file before submitting.');
  };

  const uploadFile = async (selectedFile) => {
    const formData = new FormData();
    formData.append('file', selectedFile);
    try {
      const response = await axios.post(`${process.env.REACT_APP_BACKEND_URL}/upload`, formData, getAxiosConfig());
      handleUploadSuccess(response.data);
    } catch (error) {
      handleError(error);
    }
  };

  const getAxiosConfig = () => ({
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  });

  const handleUploadSuccess = (responseData) => {
    alert('File uploaded successfully');
    console.log(responseData);
  };

  const handleError = (error) => {
    console.error('Error uploading file:', error);
    alert('Error uploading file');
  };

  return (
    <div>
      <h2>Upload Trade Data File</h2>
      <form onSubmit={handleSubmit}>
        <input type="file" onChange={handleFileChange} accept=".csv" />
        <button type="submit">Upload</button>
      </form>
    </div>
  );
};

export default Trade  DataUpload;