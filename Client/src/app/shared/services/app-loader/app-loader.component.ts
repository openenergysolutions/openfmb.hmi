// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component } from "@angular/core";
import { MatDialogRef } from "@angular/material/dialog";

@Component({
    selector: "app-app-loader",
    templateUrl: "./app-loader.component.html",
    styleUrls: ["./app-loader.component.scss"],
    standalone: false
})
export class AppLoaderComponent {
  title;
  message;
  constructor(public dialogRef: MatDialogRef<AppLoaderComponent>) {}
}
