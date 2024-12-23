import { BaseResponse } from '../helper/baseResponse.js';
import { Logger } from '../helper/logger.js';

const users = [];
const userService = {
    getAllUsers: async () => {
        return users;
    },
};

export const getUser = async (req, res) => {
    const contextAuthController = 'UserController';
    try {
        const allUsers = await userService.getAllUsers();
        Logger.info(`${contextAuthController} | getUser`, allUsers);
        BaseResponse(res, 'Users fetched successfully', 'success', { data: allUsers });
    } catch (error) {
        Logger.error(`${contextAuthController} | Error: ${error.message} | Stack: ${error.stack}`);
        res.status(500).send('Internal Server Error');
    }
};
