import { getUser } from "../controller/authController.js";
const basePrefix = '/hyper';

export const routes = [
    { path: `${basePrefix}`, method: 'GET',  handler: (req, res) => {
        res.json('Welcome to Hyper Express');
    }},
    { path: `${basePrefix}/users`, method: 'GET', handler: getUser },
];
