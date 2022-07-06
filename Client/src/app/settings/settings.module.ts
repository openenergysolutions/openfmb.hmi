// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SettingsRoutingModule } from './settings-routing.module';
import { AppSettingsComponent } from './app-settings/app-settings.component';
import { TagsComponent } from './tags/tags.component';
import { SharedComponentsModule } from '../shared/components/shared-components.module';
import { SharedMaterialModule } from '../shared/shared-material.module';
import { SharedModule } from '../shared/shared.module';
import { NgxDatatableModule } from '@swimlane/ngx-datatable';
import { DevicesComponent } from './devices/devices.component';
import { DeviceDialogsComponent } from './devices/dialogs/devicedialogs.component';


@NgModule({
  declarations: [AppSettingsComponent, TagsComponent, DevicesComponent, DeviceDialogsComponent],
  imports: [
    CommonModule,
    SettingsRoutingModule,
    SharedComponentsModule,
    SharedMaterialModule,
    SharedModule,
    NgxDatatableModule
  ]
})
export class SettingsModule { }
