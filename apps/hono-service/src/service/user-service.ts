export interface User {
  id: number;
  name: string;
  email: string;
}

const users: User[] = [
    { id: 1, name: 'John Doe', email: 'john.doe@example.com' },
    { id: 2, name: 'Jane Smith', email: 'jane.smith@example.com' }
  ];
export const getAllUsers = (): User[] => users;

export const addUser = (user: Omit<User, 'id'>): User => {
    const newUser: User = { id: users.length + 1, ...user };
    users.push(newUser);
    return newUser;
};

export const deleteUser = (id: number): User | null => {
    const index = users.findIndex((user) => user.id === id);
    if (index !== -1) {
        return users.splice(index, 1)[0];
    }
    return null;
};
