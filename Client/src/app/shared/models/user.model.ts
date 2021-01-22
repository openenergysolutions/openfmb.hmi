export const Authorization = {
  authRoles: {    
    admin: 'Admin',
    engineer: 'Engineer',
    viewer: 'viewer'
  },
  canEditDiagram: (role: string) => {
    return role === Authorization.authRoles.admin || role === Authorization.authRoles.engineer;
  },  
  canUpdateSettings: (role: string) => {
    return role === Authorization.authRoles.admin;
  }
}

export interface User {
  id?: string;
  displayname?: string;  
  role?: string;
  username?: string;
  pwd?: string;
}
