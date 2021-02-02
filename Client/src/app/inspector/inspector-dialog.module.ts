import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { InspectorDialogRoutingModule } from './inspector-dialog-routing.module';
import { InspectorDialogComponent } from './inspector-dialog.component';


@NgModule({
  declarations: [InspectorDialogComponent],
  imports: [
    CommonModule,
    InspectorDialogRoutingModule
  ]
})
export class InspectorDialogModule { }
