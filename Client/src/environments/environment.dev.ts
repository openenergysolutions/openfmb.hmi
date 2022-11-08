// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

// This file can be replaced during build by using the `fileReplacements` array.
// `ng build --prod` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

import envSettings from '../assets/env.json';

export const environment = {
  production: false,
  apiUrl: envSettings.apiUrl,
  ws: envSettings.ws,
  auth: {
    audience: envSettings.auth_audience,
    client_id: envSettings.auth_client_id,
    domain: envSettings.auth_domain,
    scope: envSettings.auth_scope,
  }
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/dist/zone-error';  // Included with Angular CLI.
