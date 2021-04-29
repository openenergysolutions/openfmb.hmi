// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DiagramsComponent } from './diagrams.component'

const routes: Routes = [
  {
    path: '',
    children: [{
      path: 'diagrams',
      component: DiagramsComponent,
      data: { title: 'DIAGRAMS' }    
    }]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class DiagramsRoutingModule { }
