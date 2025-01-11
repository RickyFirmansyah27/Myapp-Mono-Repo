import { getUser } from '../controller/usersController.js';

const userRoute = [
  {
    method: "GET",
    path: "/users",
    handler: getUser,
  },
];

export default userRoute;
