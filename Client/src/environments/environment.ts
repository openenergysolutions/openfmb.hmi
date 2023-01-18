// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

// The file contents for the current environment will overwrite these during build.
// The build system defaults to the dev environment which uses `environment.ts`, but if you do
// `ng build --env=prod` then `environment.prod.ts` will be used instead.
// The list of which env maps to which file can be found in `angular.json`.

import envSettings from '../assets/env_prod.json';

export const environment = {
  production: true,
  apiUrl: envSettings.apiUrl,
  ws: envSettings.ws,
  auth: {
    authorizePath: envSettings.auth_authorize_path,
    audience: envSettings.auth_audience,
    client_id: envSettings.auth_client_id,
    domain: envSettings.auth_domain,
    scope: envSettings.auth_scope,
  }
};
