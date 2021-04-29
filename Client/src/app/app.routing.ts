// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Routes } from '@angular/router';
import { MainLayoutComponent } from './shared/components/layouts/main-layout/main-layout.component';
import { AuthLayoutComponent } from './shared/components/layouts/auth-layout/auth-layout.component';
import { AuthGuard } from './shared/guards/auth.guard';
import { UserRoleGuard } from './shared/guards/user-role.guard'

export const rootRouterConfig: Routes = [
  { 
    path: '', 
    redirectTo: 'home', 
    pathMatch: 'full' 
  },
  {
    path: '', 
    component: AuthLayoutComponent,
    children: [
      { 
        path: 'sessions', 
        loadChildren: () => import('./views/sessions/sessions.module').then(m => m.SessionsModule),
        data: { title: 'Session'} 
      }
    ]
  },  
  {
    path: '',
    component: MainLayoutComponent,
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Viewer', 'Engineer', 'Admin' ]
    },
    children: [
      {
        path: '',
        loadChildren: () => import('./home/home.module').then(m => m.HomeModule)       
      }
    ]
  },
  {
    path: '',
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Viewer', 'Engineer', 'Admin' ]
    },
    children: [
      {
        path: '',
        loadChildren: () => import('./hmi/hmi.module').then(m => m.HmiModule)       
      }
    ]
  },
  {
    path: '',
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Viewer', 'Engineer', 'Admin' ]
    },
    children: [
      {
        path: '',
        loadChildren: () => import('./inspector/inspector-dialog.module').then(m => m.InspectorDialogModule)       
      }
    ]
  },    
  {
    path: '',
    component: MainLayoutComponent,
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Viewer', 'Engineer', 'Admin' ]
    },
    children: [
      {
        path: '',
        loadChildren: () => import('./diagrams/diagrams.module').then(m => m.DiagramsModule)        
      }
    ]
  },  
  {
    path: '', 
    component: MainLayoutComponent,   
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Engineer', 'Admin' ]
    },    
    children: [
      {
        path: '',
        loadChildren: () => import('./data-connect/data-connect.module').then(m => m.DataConnectModule)        
      }
    ]
  },
  {
    path: '',    
    canActivate: [AuthGuard, UserRoleGuard],
    component: MainLayoutComponent,
    data: 
    { 
      roles: [ 'Admin' ]
    }, 
    children: [
      {
        path: 'settings',
        loadChildren: () => import('./settings/settings.module').then(m => m.SettingsModule)        
      }
    ]
  },
  {
    path: '',    
    canActivate: [AuthGuard, UserRoleGuard],
    data: 
    { 
      roles: ['Engineer', 'Admin' ]
    },       
    children: [
      {
        path: '',
        loadChildren: () => import('./designer/designer.module').then(m => m.DesignerModule)        
      }
    ]
  },
  { 
    path: '**', 
    redirectTo: 'sessions/404'
  }
];

