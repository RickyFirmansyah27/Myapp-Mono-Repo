// src/routes/index.ts
import "reflect-metadata";
import { Hono } from 'hono';
import UserRoutes from './user-route';

const router = new Hono();
const basePath = '/hono';

// Mount routes with base path
router.route(`${basePath}/user`, UserRoutes);

export { router as routes };