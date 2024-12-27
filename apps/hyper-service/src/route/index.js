import userRoute from './userRoute.js';

const basePath = '/api';

export const routes = [
    { path: `${basePath}/users`, handler: userRoute },
];
