import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DiagramsRoutingModule } from './diagrams-routing.module';
import { DiagramsComponent } from './diagrams.component';
import { SharedMaterialModule } from '../shared/shared-material.module';
import { NgxDatatableModule } from '@swimlane/ngx-datatable';
import { DialogsComponent } from './dialogs/dialogs.component';
import { SharedComponentsModule } from '../shared/components/shared-components.module';
import { SharedModule } from '../shared/shared.module';
import { NgxMatColorPickerModule, MAT_COLOR_FORMATS, NGX_MAT_COLOR_FORMATS } from '@angular-material-components/color-picker'

@NgModule({
  declarations: [DiagramsComponent, DialogsComponent],
  imports: [
    CommonModule,
    DiagramsRoutingModule,
    SharedMaterialModule,
    SharedComponentsModule,
    SharedModule,
    NgxDatatableModule,
    NgxMatColorPickerModule    
  ],
  providers: [
    { 
      provide: MAT_COLOR_FORMATS, 
      useValue: NGX_MAT_COLOR_FORMATS 
    }
  ],
})
export class DiagramsModule { }
