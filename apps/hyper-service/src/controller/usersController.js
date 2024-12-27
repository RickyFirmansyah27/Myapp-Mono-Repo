import { BaseResponse } from '../helper/baseResponse.js';
import { Logger } from '../helper/logger.js';

const users = [
    {
        id: 1,
        name: 'John Doe',
        email: 'john.doe@example.com',
        age: 30,
    },
    {
        id: 2,
        name: 'Jane Smith',
        email: 'jane.smith@example.com',
        age: 25,
    },
];

const userService = {
    getAllUsers: async () => {
        return users;
    },
};

export const getUser = async (req, res) => {
    const contextUserController = 'UserController';
    try {
        const allUsers = await userService.getAllUsers();
        Logger.info(`${contextUserController} | getUser`, allUsers);
        BaseResponse(res, 'Users fetched successfully', 'success', allUsers);
    } catch (error) {
        Logger.error(`${contextUserController} | Error: ${error.message} | Stack: ${error.stack}`);
        res.status(500).send('Internal Server Error');
    }
};
