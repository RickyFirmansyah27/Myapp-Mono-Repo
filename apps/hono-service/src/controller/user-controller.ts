import { BaseResponse, Logger } from '../helper';
import { getAllUsers, addUser, deleteUser } from '../service/user-service';
import { Context } from 'hono';

const contextLogger = 'UserController';

export const getUsersHandler = (c: Context) => {
  try {
    const users = getAllUsers();
    Logger.info(`${contextLogger} | users: ${JSON.stringify(users)}`);
    return BaseResponse(c, 'User fetched successfully', 'success', { users: users });
  } catch (error) {
    Logger.error(`${contextLogger} | getUser | error: ${(error as Error).message}`);
    return BaseResponse(c, 'Failed to fetch users', 'internalServerError');
  }
};

export const addUserHandler = async (c: Context) => {
  try {
    const body = await c.req.json<{ name: string; email: string }>();
    const user = addUser(body);
    Logger.info(`${contextLogger} | addUser`, user);
    return BaseResponse(c, 'User created successfully', 'created', { data: user });
  } catch (error) {
    Logger.error(`${contextLogger} | addUser | error: ${(error as Error).message}`);
    return BaseResponse(c, 'Failed to create user', 'internalServerError');
  }
};

export const deleteUserHandler = (c: Context) => {
  try {
    const id = parseInt(c.req.param('id'), 10);
    const deletedUser = deleteUser(id);
    if (deletedUser) {
      Logger.info(`${contextLogger} | deleteUser`, { id });
      return BaseResponse(c, 'User deleted successfully', 'success', { data: [] });
    }
    return BaseResponse(c, 'User not found', 'notFound');
  } catch (error) {
    Logger.error(`${contextLogger} | deleteUser | error: ${(error as Error).message}`);
    return BaseResponse(c, 'Failed to delete user', 'internalServerError');
  }
};
