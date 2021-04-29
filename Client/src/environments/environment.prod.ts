import envSettings from '../assets/env_prod.json';

export const environment = {
  production: false,
  apiUrl: envSettings.apiUrl,
  ws: envSettings.ws
};