import { InspectorDialogComponent } from './inspector-dialog.component';
import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';


const routes: Routes = [
  {
    path: '',
    children: [{
      path: 'inspector',
      component: InspectorDialogComponent,
      data: { title: 'INSPECTOR' }
    }]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class InspectorDialogRoutingModule { }
