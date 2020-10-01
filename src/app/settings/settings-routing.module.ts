import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AppSettingsComponent } from './app-settings/app-settings.component';
import { UsersComponent } from './users/users.component';
import { TagsComponent } from './tags/tags.component';

const routes: Routes = [
  {
    path: '',
    children: [
      {
        path: 'general',
        component: AppSettingsComponent,
        data: { title: 'GENERAL', breadcrumb: 'GENERAL' }
      }, {
        path: 'users',
        component: UsersComponent,
        data: { title: 'USERS', breadcrumb: 'USERS' }
      }, {
        path: 'tags',
        component: TagsComponent,
        data: { title: 'TAGS', breadcrumb: 'TAGS' }
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class SettingsRoutingModule { }
