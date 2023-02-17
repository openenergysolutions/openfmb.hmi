// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { NgModule, ErrorHandler } from '@angular/core';
import { RouterModule } from '@angular/router';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { PerfectScrollbarModule, PERFECT_SCROLLBAR_CONFIG, PerfectScrollbarConfigInterface } from 'ngx-perfect-scrollbar';

import { rootRouterConfig } from './app.routing';
import { SharedModule } from './shared/shared.module';
import { SharedMaterialModule } from './shared/shared-material.module';
import { AppComponent } from './app.component';

import { HttpClient, HttpClientModule, HTTP_INTERCEPTORS } from '@angular/common/http';
import { TranslateModule, TranslateLoader } from '@ngx-translate/core';
import { TranslateHttpLoader } from '@ngx-translate/http-loader';
import { ErrorHandlerService } from './shared/services/error-handler.service';

import { StoreModule } from '@ngrx/store';
import { reducers, metaReducers } from './store/reducers';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { environment } from '../environments/environment';
import { EffectsModule } from '@ngrx/effects';
import { AppEffects } from './app.effects';
import { StoreRouterConnectingModule } from '@ngrx/router-store';
import { EntityDataModule } from '@ngrx/data';
import { entityConfig } from './entity-metadata';
import { NgxSpinnerModule } from 'ngx-spinner';
import { AuthModule } from '@auth0/auth0-angular';
import { ErrorInterceptor } from './core/helpers/error-interceptor';
import { LoadingInterceptor } from './core/helpers/loading-interceptor';
import { DesignerEffects } from './store/effects/designer.effects';
import { WebSocketModule } from './web-socket/web-socket.module';
import { NgxDatatableModule } from '@swimlane/ngx-datatable';
import { CustomAuthInterceptor } from './shared/httpinterceptors/custom-auth-interceptor';

// AoT requires an exported function for factories
export function HttpLoaderFactory(httpClient: HttpClient) {
  return new TranslateHttpLoader(httpClient);
}

const DEFAULT_PERFECT_SCROLLBAR_CONFIG: PerfectScrollbarConfigInterface = {
  suppressScrollX: true
};

@NgModule({
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    SharedModule,
    SharedMaterialModule,
    HttpClientModule,
    PerfectScrollbarModule,
    TranslateModule.forRoot({
      loader: {
        provide: TranslateLoader,
        useFactory: HttpLoaderFactory,
        deps: [HttpClient]
      }
    }),
    RouterModule.forRoot(rootRouterConfig, { useHash: false, relativeLinkResolution: 'legacy' }),
    StoreModule.forRoot(reducers, {
      metaReducers,
      runtimeChecks: {
        strictStateImmutability: true,
        strictActionImmutability: true
      }
    }),
    StoreDevtoolsModule.instrument({ maxAge: 25, logOnly: environment.production }),
    EffectsModule.forRoot([AppEffects]),
    StoreRouterConnectingModule.forRoot(),
    EntityDataModule.forRoot(entityConfig),
    EffectsModule.forFeature([DesignerEffects]),
    NgxSpinnerModule,
    NgxDatatableModule,
    HttpClientModule,
    AuthModule.forRoot({
      // Should these be in the environment?
      domain: environment.auth.domain,
      clientId: environment.auth.client_id,
      // To let us refresh a page, cache in local storage
      cacheLocation: 'localstorage',
      // Request this audience at user authentication time
      audience: environment.auth.audience,
      // New authorizePath option
      authorizePath: environment.auth.authorize_path,
      tokenPath: environment.auth.token_path,
      redirectUri: window.location.origin,
      httpInterceptor: {
        allowedList: [
          {
            uri: environment.apiUrl + '*',
            tokenOptions: {
              // detailedResponse: true,
              audience: environment.auth.audience,
              scope: environment.auth.scope,
            }
          }
        ]
      },
      useFormData: true,
      disableAuth0Client: true,
    }),
    WebSocketModule.config({
      url: environment.ws
    })
  ],
  declarations: [AppComponent],
  providers: [
    { provide: ErrorHandler, useClass: ErrorHandlerService },
    { provide: PERFECT_SCROLLBAR_CONFIG, useValue: DEFAULT_PERFECT_SCROLLBAR_CONFIG },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: ErrorInterceptor,
      multi: true
    },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: LoadingInterceptor,
      multi: true
    },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: CustomAuthInterceptor,
      multi: true
    },
    {
      provide: Window,
      useValue: window
    }
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
