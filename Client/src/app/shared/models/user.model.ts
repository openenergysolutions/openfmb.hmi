// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

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
  },
  canControl: (role: string) => {
    return role === Authorization.authRoles.admin || role === Authorization.authRoles.engineer;
  }
}

export interface User {
  id?: string;
  displayname?: string;  
  role?: string;
  username?: string;
  pwd?: string;
}
