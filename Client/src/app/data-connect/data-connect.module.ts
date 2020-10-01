import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SharedComponentsModule } from '../shared/components/shared-components.module';
import { SharedModule } from '../shared/shared.module';

import { DataConnectRoutingModule } from './data-connect-routing.module';
import { DataConnectComponent } from './data-connect.component';
import { SharedMaterialModule } from '../shared/shared-material.module';
import { DragDropModule } from '@angular/cdk/drag-drop';

@NgModule({
  declarations: [DataConnectComponent],
  imports: [
    CommonModule,
    DataConnectRoutingModule,
    SharedComponentsModule,
    SharedMaterialModule,
    SharedModule,
    DragDropModule
  ]
})
export class DataConnectModule { }
