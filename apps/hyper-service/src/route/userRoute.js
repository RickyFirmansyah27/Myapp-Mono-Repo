import HyperExpress from 'hyper-express';
import { getUser } from '../controller/usersController.js';

const userRoute = new HyperExpress.Router();

userRoute.route('/').get(getUser);

export default userRoute;
