import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { DesignerRoutingModule } from './hmi-routing.module';
import { HmiComponent } from './hmi.component';
import { SharedModule } from '../shared/shared.module';
import { HeaderToolComponent } from './header-tool/header-tool.component';
import { PropertiesDialogComponent } from './dialogs/properties-dialog/properties-dialog.component';
import { SwitchgearDialogComponent } from './dialogs/switchgear-dialog/switchgear-dialog.component';
import { SharedMaterialModule } from '../shared/shared-material.module';

@NgModule({
  declarations: [
    HmiComponent,
    HeaderToolComponent,
    PropertiesDialogComponent,
    SwitchgearDialogComponent    
  ],
  imports: [
    CommonModule,
    DesignerRoutingModule,
    SharedModule,
    SharedMaterialModule
  ],
  entryComponents: [
    PropertiesDialogComponent,
    SwitchgearDialogComponent
  ]
})
export class HmiModule { }
