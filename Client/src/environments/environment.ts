// The file contents for the current environment will overwrite these during build.
// The build system defaults to the dev environment which uses `environment.ts`, but if you do
// `ng build --env=prod` then `environment.prod.ts` will be used instead.
// The list of which env maps to which file can be found in `angular.json`.

export const environment = {
  production: false,
  apiUrl: 'http://10.10.200.99:32771/',
  ws: 'ws://10.10.200.99:32771/hmi/'
};
