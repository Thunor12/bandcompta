import axios from "axios";

import { transactionTemplate } from "./transactionModel";

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

let config_delete = {
  method: 'delete',
  maxBodyLength: Infinity,
  url: 'http://127.0.0.1:8080/transactions',
  responseType: 'json',
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    "Access-Control-Allow-Origin": "http://127.0.0.1:8080/transactions",
    "Access-Control-Allow-Methods": "DELETE"
  },
  data: {

  }
};

const getTransactions = async () => {
  let tab = [];

  let response = await axios.request(config_get);

  response.data.forEach(rt => {
    tab.push(rt);
  });

  return tab;
};

const initDB = async () => {
  await getTransactions();
};

const addTransaction = async (transaction) => {

  console.log("Adding transcation: ", transaction);

  config_post.data = transaction;
  await axios.request(config_post);

  config_post.data = {};
};

const deleteTransaction = async (tr) => {
  console.log("Transaction to delete: ", tr);

  config_delete.data = tr;
  await axios.request(config_delete);

  config_delete.data = {};
};

export { initDB, addTransaction, getTransactions, deleteTransaction };