// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DataConnectComponent } from './data-connect.component';

const routes: Routes = [
  {
    path: '',
    children: [{
      path: 'data-connect',
      component: DataConnectComponent,
      data: { title: 'DATA' }    
    }]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class DataConnectRoutingModule { }
