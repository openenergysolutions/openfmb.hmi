// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import envSettings from '../assets/env_prod.json';

export const environment = {
  production: false,
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
