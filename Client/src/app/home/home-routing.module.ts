import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './home.component'

const routes: Routes = [
  {
    path: '',
    children: [{
      path: 'home',
      component: HomeComponent,
      data: { title: 'HOME' }   
    }]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class HomeRoutingModule { }
