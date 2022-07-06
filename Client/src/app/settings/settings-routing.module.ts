// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AppSettingsComponent } from './app-settings/app-settings.component';
import { TagsComponent } from './tags/tags.component';
import { DevicesComponent } from './devices/devices.component';

const routes: Routes = [
  {
    path: '',
    children: [
      {
        path: 'general',
        component: AppSettingsComponent,
        data: { title: 'GENERAL' }
      }, {
        path: 'tags',
        component: TagsComponent,
        data: { title: 'TAGS' }
      }, {
        path: 'devices',
        component: DevicesComponent,
        data: { title: 'DEVICES' }
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class SettingsRoutingModule { }
