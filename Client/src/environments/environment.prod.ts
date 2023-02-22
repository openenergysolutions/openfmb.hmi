// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import envSettings from '../assets/env_prod.json';

export const environment = {
  production: true,
  apiUrl: envSettings.apiUrl,
  ws: envSettings.ws,
  auth: {
    authorize_path: envSettings.auth_authorize_path,
    token_path: envSettings.auth_token_path,
    audience: envSettings.auth_audience,
    client_id: envSettings.auth_client_id,
    domain: envSettings.auth_domain,
    scope: envSettings.auth_scope,
  }
};
