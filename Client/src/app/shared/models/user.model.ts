// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

type authRole = 'SuperUser' | 'Engineer' | 'Viewer';

const canX = (roles: Array<string>, authorizedRoles: Array<authRole>) => {
  if (roles) {
    return authorizedRoles.some(role => roles.includes(role));
  } else {
    return false;
  }
};

export const Authorization = {
  authRoles: {
    admin: 'SuperUser' as authRole,
    engineer: 'Engineer' as authRole,
    viewer: 'Viewer' as authRole,
  },
  canEditDiagram: (roles: string[]) => {
    return canX(roles, [Authorization.authRoles.admin, Authorization.authRoles.engineer]);
  },
  canUpdateSettings: (roles: Array<string>) => {
    return canX(roles, [Authorization.authRoles.admin]);
  }
}
