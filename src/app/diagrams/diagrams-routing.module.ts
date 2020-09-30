import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DiagramsComponent } from './diagrams.component'

const routes: Routes = [
  {
    path: '',
    children: [{
      path: '',
      component: DiagramsComponent      
    }]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class DiagramsRoutingModule { }
