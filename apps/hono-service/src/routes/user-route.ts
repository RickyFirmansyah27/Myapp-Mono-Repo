import { Hono } from 'hono';
import { getUsersHandler, addUserHandler, deleteUserHandler } from '../controller/user-controller';

const userRoutes = new Hono();

userRoutes.get('/', getUsersHandler);
userRoutes.post('/', addUserHandler);
userRoutes.delete('/:id', deleteUserHandler);

export default userRoutes;
