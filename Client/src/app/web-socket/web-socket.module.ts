// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { ModuleWithProviders, NgModule } from '@angular/core';
import { WebSocketConfig } from '../core/models/webSocket';
import { config } from './web-socket.config';
import { CommonModule } from '@angular/common';

@NgModule({
  imports: [
    CommonModule
  ],
  declarations: [],
})
export class WebSocketModule {
  public static config(wsConfig: WebSocketConfig): ModuleWithProviders<WebSocketModule> {
    return {
      ngModule: WebSocketModule,
      providers: [{ provide: config, useValue: wsConfig }]
    };
  }
}
