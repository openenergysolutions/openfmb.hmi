// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Routes } from "@angular/router";

import { SigninComponent } from "./signin/signin.component";
import { NotFoundComponent } from "./not-found/not-found.component";
import { ErrorComponent } from "./error/error.component";

export const SessionsRoutes: Routes = [
  {
    path: "",
    children: [        
      {
        path: "signin",
        component: SigninComponent,
        data: { title: "Signin" }
      },             
      {
        path: "404",
        component: NotFoundComponent,
        data: { title: "Not Found" }
      },
      {
        path: "error",
        component: ErrorComponent,
        data: { title: "Error" }
      }
    ]
  }
];
