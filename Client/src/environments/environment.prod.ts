// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export const environment = {
  production: false,
  apiUrl: window.location.protocol + "//" + window.location.host + "/",
  ws:
    window.location.protocol == "https:"
      ? "wss://" +
        window.location.hostname +
        ":" +
        window.location.port +
        "/data/"
      : "ws://" +
        window.location.hostname +
        ":" +
        window.location.port +
        "/data/",
};
