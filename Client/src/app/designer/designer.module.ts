import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { DesignerRoutingModule } from './designer-routing.module';
import { DesignerComponent } from './designer.component';
import { SharedModule } from '../shared/shared.module';
import { HeaderToolComponent } from './header-tool/header-tool.component';
import { DocumentToolbarComponent } from './document-toolbar/document-toolbar.component';
import { PropertiesDialogComponent } from './dialogs/properties-dialog/properties-dialog.component';
import { SharedMaterialModule } from '../shared/shared-material.module';
import { NgxMatColorPickerModule, MAT_COLOR_FORMATS, NGX_MAT_COLOR_FORMATS } from '@angular-material-components/color-picker'

@NgModule({
  declarations: [
    DesignerComponent,
    HeaderToolComponent,
    DocumentToolbarComponent,
    PropertiesDialogComponent    
  ],
  imports: [
    CommonModule,
    DesignerRoutingModule,
    SharedModule,
    SharedMaterialModule,
    NgxMatColorPickerModule
  ],
  providers: [
    { 
      provide: MAT_COLOR_FORMATS, 
      useValue: NGX_MAT_COLOR_FORMATS 
    }
  ],
  entryComponents: [
    PropertiesDialogComponent
  ]
})
export class DesignerModule { }
