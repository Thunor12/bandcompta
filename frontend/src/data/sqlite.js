import axios from "axios";

import { transactionTemplate } from "./transactionModel";

// const Url = '127.0.0.1:8080/transactions/';

const config_get = {
  method: 'get',
  maxBodyLength: Infinity,
  url: 'http://127.0.0.1:8080/transactions',
  responseType: 'json',
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    "Access-Control-Allow-Origin": "http://127.0.0.1:8080/transactions",
    "Access-Control-Allow-Methods": "GET"
  }
};

let config_post = {
  method: 'post',
  maxBodyLength: Infinity,
  url: 'http://127.0.0.1:8080/transactions',
  responseType: 'json',
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    "Access-Control-Allow-Origin": "http://127.0.0.1:8080/transactions",
    "Access-Control-Allow-Methods": "POST"
  },
  data: {

  }
};

let db = [];

const initDB = () => {

  if (db.length === 0) {
    db.push(transactionTemplate);
  }
};

const addTransaction = async (transaction) => {

  console.log("Adding transcation: ", transaction);

  // await axios.post('/user', transaction, config_post);

  config_post.data = transaction;
  await axios.request(config_post);

  config_post.data = {};
};

const getTransactions = async () => {
  let tab = [];

  let response = await axios.request(config_get);

  response.data.forEach(rt => {
    tab.push(rt);
  });

  return tab;
};

const deleteTransaction = (id) => {

  console.log("ID to delete: ", id);
};

export { initDB, addTransaction, getTransactions, deleteTransaction };