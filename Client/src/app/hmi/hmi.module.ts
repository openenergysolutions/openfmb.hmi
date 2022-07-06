// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { DesignerRoutingModule } from './hmi-routing.module';
import { HmiComponent } from './hmi.component';
import { SharedModule } from '../shared/shared.module';
import { HeaderToolComponent } from './header-tool/header-tool.component';
import { PropertiesDialogComponent } from './dialogs/properties-dialog/properties-dialog.component';
import { SwitchgearDialogComponent } from './dialogs/switchgear-dialog/switchgear-dialog.component';
import { RegulatorDialogComponent } from './dialogs/regulator-dialog/regulator-dialog.component';
import { GenericDialogComponent } from './dialogs/generic-dialog/generic-dialog.component';
import { ControlDialogComponent } from './dialogs/control-dialog/control-dialog.component';
import { SharedMaterialModule } from '../shared/shared-material.module';

@NgModule({
    declarations: [
        HmiComponent,
        HeaderToolComponent,
        PropertiesDialogComponent,
        SwitchgearDialogComponent,
        RegulatorDialogComponent,
        ControlDialogComponent,
        GenericDialogComponent
    ],
    imports: [
        CommonModule,
        DesignerRoutingModule,
        SharedModule,
        SharedMaterialModule
    ]
})
export class HmiModule { }
