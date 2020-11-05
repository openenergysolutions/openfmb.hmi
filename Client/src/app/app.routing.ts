import { Routes } from '@angular/router';
import { MainLayoutComponent } from './shared/components/layouts/main-layout/main-layout.component';
import { AuthLayoutComponent } from './shared/components/layouts/auth-layout/auth-layout.component';
import { AuthGuard } from './shared/guards/auth.guard';

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
    canActivate: [AuthGuard],
    children: [
      {
        path: '',
        loadChildren: () => import('./home/home.module').then(m => m.HomeModule)       
      }
    ]
  },
  {
    path: '',
    canActivate: [AuthGuard],
    children: [
      {
        path: '',
        loadChildren: () => import('./hmi/hmi.module').then(m => m.HmiModule)       
      }
    ]
  },    
  {
    path: '',
    component: MainLayoutComponent,
    canActivate: [AuthGuard],
    children: [
      {
        path: '',
        loadChildren: () => import('./diagrams/diagrams.module').then(m => m.DiagramsModule)        
      }
    ]
  },  
  {
    path: '',    
    canActivate: [AuthGuard],
    component: MainLayoutComponent,
    children: [
      {
        path: '',
        loadChildren: () => import('./data-connect/data-connect.module').then(m => m.DataConnectModule)        
      }
    ]
  },
  {
    path: '',    
    canActivate: [AuthGuard],
    component: MainLayoutComponent,
    children: [
      {
        path: 'settings',
        loadChildren: () => import('./settings/settings.module').then(m => m.SettingsModule)        
      }
    ]
  },
  {
    path: '',    
    canActivate: [AuthGuard],    
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

